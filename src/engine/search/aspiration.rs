use crate::chess::game::Game;
use crate::engine::eval::Eval;
use crate::engine::search::time_control::TimeStrategy;
use crate::engine::search::transposition::SearchTranspositionTable;
use crate::engine::search::{negamax, params, Control, SearchState};

pub fn aspiration_search(
    game: &mut Game,
    depth: u8,
    eval: Option<Eval>,
    tt: &mut SearchTranspositionTable,
    state: &mut SearchState,
    time_control: &TimeStrategy,
    control: &impl Control,
) -> Result<Eval, ()> {
    let mut alpha = Eval::MIN;
    let mut beta = Eval::MAX;
    let mut width = params::ASPIRATION_WINDOW_SIZE;

    if depth >= params::ASPIRATION_MIN_DEPTH {
        let eval = eval.unwrap();
        alpha = Eval::max(Eval::MIN, eval - width);
        beta = Eval::min(Eval::MAX, eval + width);
    }

    loop {
        let Ok(eval) = negamax::negamax(
            game,
            alpha,
            beta,
            depth,
            0,
            tt,
            time_control,
            state,
            control,
        ) else {
            return Err(());
        };

        width = width * 2;

        if eval <= alpha {
            alpha = alpha - width;
        } else if eval > beta {
            beta = beta + width;
        } else {
            return Ok(eval);
        }
    }
}
