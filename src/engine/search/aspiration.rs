use crate::chess::game::Game;
use crate::engine::eval::Eval;
use crate::engine::search::time_control::TimeStrategy;
use crate::engine::search::transposition::SearchTranspositionTable;
use crate::engine::search::{negamax, params, Control, SearchState};

struct Window {
    alpha: Eval,
    beta: Eval,

    widenings: u8,
    width: Option<Eval>,
}

impl Window {
    pub fn no_window() -> Self {
        Self {
            alpha: Eval::MIN,
            beta: Eval::MAX,

            widenings: 0,
            width: None,
        }
    }

    pub fn around(eval: Eval, width: Eval) -> Self {
        Self {
            alpha: Self::clamp_alpha(eval - width),
            beta: Self::clamp_beta(eval + width),

            widenings: 0,
            width: Some(width),
        }
    }

    pub fn widen_down(&mut self) {
        if let Some(width) = self.width {
            if self.widenings <= 2 {
                self.increase_window_widening_rate();
                self.alpha = Self::clamp_alpha(self.alpha - width);
            } else {
                self.alpha = Eval::MIN;
                self.beta = Eval::MAX;
            }
        }
    }

    pub fn widen_up(&mut self) {
        if let Some(width) = self.width {
            if self.widenings <= 2 {
                self.increase_window_widening_rate();
                self.beta = Self::clamp_beta(self.beta + width);
            } else {
                self.alpha = Eval::MIN;
                self.beta = Eval::MAX;
            }
        }
    }

    fn increase_window_widening_rate(&mut self) {
        self.widenings += 1;
        self.width = Some(self.width.unwrap() * 2);
    }

    fn clamp_alpha(eval: Eval) -> Eval {
        std::cmp::max(Eval::MIN, eval)
    }

    fn clamp_beta(eval: Eval) -> Eval {
        std::cmp::min(Eval::MAX, eval)
    }
}

pub fn aspiration_search(
    game: &mut Game,
    depth: u8,
    eval: Option<Eval>,
    tt: &mut SearchTranspositionTable,
    state: &mut SearchState,
    time_control: &TimeStrategy,
    control: &impl Control,
) -> Result<Eval, ()> {
    let mut window = if depth < params::ASPIRATION_MIN_DEPTH {
        Window::no_window()
    } else {
        Window::around(eval.unwrap(), params::ASPIRATION_WINDOW_SIZE)
    };

    loop {
        let Ok(eval) = negamax::negamax(
            game,
            window.alpha,
            window.beta,
            depth,
            0,
            tt,
            time_control,
            state,
            control,
        ) else {
            return Err(());
        };

        if eval <= window.alpha {
            window.widen_down();
        } else if eval >= window.beta {
            window.widen_up();
        } else {
            return Ok(eval);
        }
    }
}
