// A huge thanks to GediminasMasaitis and Andrew Grant.
// This code borrows heavily from https://github.com/GediminasMasaitis/texel-tuner
// which is in turn based on https://github.com/AndyGrant/Ethereal/blob/master/Tuning.pdf

use crate::chess::game::Game;
use crate::utils::tuner::parameters::Parameters;
use crate::utils::tuner::trace::Trace;
use crate::utils::tuner::tuner_eval::TunerEval;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::path::Path;

mod parameters;
mod trace;
mod tuner_eval;

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn numeric_outcome(&self) -> f32 {
        match self {
            Self::Win => 1.0,
            Self::Draw => 0.5,
            Self::Loss => 0.0,
        }
    }
}

#[derive(Clone)]
struct NonZeroCoefficient {
    idx: usize,
    value: f32,
}

impl NonZeroCoefficient {
    pub fn new(idx: usize, value: f32) -> Self {
        Self { idx, value }
    }
}

struct Entry {
    outcome: Outcome,
    coefficients: Vec<NonZeroCoefficient>,

    midgame_percentage: f32,
    endgame_percentage: f32,
}

fn start_progress_bar(size: usize, label: &str) -> ProgressBar {
    let p = ProgressBar::new(size as u64);
    p.set_prefix(label.to_owned());
    p.set_style(
        ProgressStyle::with_template(
            "{prefix} [{wide_bar:.cyan/blue}] {pos}/{len} ({elapsed} {per_sec} ETA {eta})",
        )
        .unwrap()
        .progress_chars("#>-"),
    );
    p
}

fn load_entries_from_file(path: &Path) -> Vec<Entry> {
    let file_contents = std::fs::read_to_string(path).expect("Unable to read file");
    let lines = file_contents.lines().collect::<Vec<&str>>();

    let number_of_positions = lines.len();

    let parsing_progress = start_progress_bar(number_of_positions, "Loading positions");
    let mut parse_results: Vec<(Game, Outcome)> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let (fen_str, outcome_str) = line.split_once('[').expect("Unexpected file format");
        let fen_str = fen_str.trim();
        let outcome_str = outcome_str.trim().replace(']', "");

        let game = Game::from_fen(fen_str).expect("Unexpected fen");

        let outcome = match outcome_str.as_str() {
            "1.0" => Outcome::Win,
            "0.5" => Outcome::Draw,
            "0.0" => Outcome::Loss,
            _ => panic!("Unexpected outcome format"),
        };

        parse_results.push((game, outcome));

        if i % 1000 == 0 {
            parsing_progress.set_position(i as u64);
        }
    }

    parsing_progress.finish();

    let coefficients_progress = start_progress_bar(number_of_positions, "Calculating coefficients");
    let mut entries: Vec<Entry> = Vec::new();

    for (i, (game, outcome)) in parse_results.into_iter().enumerate() {
        let coefficients = Trace::for_game(&game).non_zero_coefficients();

        let midgame_percentage =
            f32::from(game.incremental_eval.phase_value) / f32::from(tuner_eval::PHASE_COUNT_MAX);
        let endgame_percentage = 1.0 - midgame_percentage;

        entries.push(Entry {
            outcome,
            coefficients,

            midgame_percentage,
            endgame_percentage,
        });

        if i % 1000 == 0 {
            coefficients_progress.set_position(i as u64);
        }
    }

    coefficients_progress.finish();

    entries
}

fn evaluate(entry: &Entry, parameters: &[TunerEval]) -> f32 {
    let mut s = TunerEval::ZERO;

    for coefficient in &entry.coefficients {
        s += parameters[coefficient.idx] * coefficient.value;
    }

    s.midgame().mul_add(
        entry.midgame_percentage,
        s.endgame() * entry.endgame_percentage,
    )
}

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + f32::exp(-x))
}

fn calculate_gradient(
    entries: &[Entry],
    parameters: &[TunerEval; Trace::SIZE],
    k: f32,
) -> [TunerEval; Trace::SIZE] {
    // Break the entries into chunks, aiming for as many chunks as we have CPUs.
    //
    // I previously tried using Rayon's .fold() but this results in a huge number of array copies
    // due to Rayon's default binary-tree-of-tasks strategy. Each leaf gets its own array, which means
    // we have to allocate a huge number of trace-sized arrays.
    // With this approach, we allocate only a single array per chunk, and sum them at the end.
    let entry_chunks = entries.chunks(entries.len() / 10).collect::<Vec<_>>();

    entry_chunks
        .par_iter()
        .map(|&entries| {
            let mut gradient = [TunerEval::ZERO; Trace::SIZE];

            for entry in entries {
                let eval = evaluate(entry, parameters);
                let sigmoid = sigmoid(k * eval / 400.0);
                let result =
                    (entry.outcome.numeric_outcome() - sigmoid) * sigmoid * (1.0 - sigmoid);

                for coefficient in &entry.coefficients {
                    gradient[coefficient.idx] += TunerEval::new(
                        entry.midgame_percentage * coefficient.value,
                        entry.endgame_percentage * coefficient.value,
                    ) * result;
                }
            }

            gradient
        })
        .reduce(
            || [TunerEval::ZERO; Trace::SIZE],
            |mut gradient: [TunerEval; Trace::SIZE], thread_gradient: [TunerEval; Trace::SIZE]| {
                for (idx, score) in thread_gradient.iter().enumerate() {
                    gradient[idx] += *score;
                }

                gradient
            },
        )
}

#[expect(clippy::cast_precision_loss, reason = "Known imprecise calculations")]
pub fn tune(path: &Path, epochs: usize) {
    rayon::ThreadPoolBuilder::new()
        .stack_size(5_000_000)
        .build_global()
        .unwrap();

    let entries = load_entries_from_file(path);

    // TODO: Using the same k as was determined by texel-tuner until we compute it here.
    let k = 2.5;

    let learning_rate = 1.0;
    let beta1 = 0.9;
    let beta2 = 0.999;

    let mut parameters: [TunerEval; Trace::SIZE] = [TunerEval::ZERO; Trace::SIZE];
    let mut momentum: [TunerEval; Trace::SIZE] = [TunerEval::ZERO; Trace::SIZE];
    let mut velocities: [TunerEval; Trace::SIZE] = [TunerEval::ZERO; Trace::SIZE];

    let epoch_progress = start_progress_bar(epochs, "Running epochs");

    for epoch in 0..epochs {
        let gradient = calculate_gradient(&entries, &parameters, k);

        for param in 0..Trace::SIZE {
            let grad = TunerEval::v(-k) / TunerEval::v(400.0) * gradient[param]
                / TunerEval::v(entries.len() as f32);
            momentum[param] = momentum[param] * beta1 + grad * (1.0 - beta1);
            velocities[param] = velocities[param] * beta2 + (grad * grad) * (1.0 - beta2);

            parameters[param] -=
                momentum[param] * learning_rate / (TunerEval::v(1e-8) + velocities[param].sqrt());
        }

        epoch_progress.set_position((epoch + 1) as u64);
    }

    let parameters = Parameters::from_array(&parameters);
    println!("{}", &parameters);
}
