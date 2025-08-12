use crate::chess::game::Game;
use crate::engine::eval::Eval;
use crate::engine::search::principal_variation::PrincipalVariation;
use crate::engine::search::{negamax, params, SearchContext};

struct Window {
    alpha: Eval,
    beta: Eval,

    width: Eval,
}

fn clamp_alpha(eval: Eval) -> Eval {
    std::cmp::max(Eval::MIN, eval)
}

fn clamp_beta(eval: Eval) -> Eval {
    std::cmp::min(Eval::MAX, eval)
}

impl Window {
    pub fn no_window() -> Self {
        Self {
            alpha: Eval::MIN,
            beta: Eval::MAX,

            width: Eval(0),
        }
    }

    pub fn around(eval: Eval, width: Eval) -> Self {
        Self {
            alpha: clamp_alpha(eval - width),
            beta: clamp_beta(eval + width),

            width,
        }
    }

    pub fn widen_down(&mut self) {
        self.increase_window_widening_rate();
        self.alpha = clamp_alpha(self.alpha - self.width);
    }

    pub fn widen_up(&mut self) {
        self.increase_window_widening_rate();
        self.beta = clamp_beta(self.beta + self.width);
    }

    fn increase_window_widening_rate(&mut self) {
        self.width = self.width + self.width / 2;
    }
}

pub fn aspiration_search(
    game: &mut Game,
    depth: u8,
    eval: Option<Eval>,
    pv: &mut PrincipalVariation,
    ctx: &mut SearchContext<'_>,
) -> Result<Eval, ()> {
    let mut window = if depth < params::ASPIRATION_MIN_DEPTH {
        Window::no_window()
    } else {
        Window::around(eval.unwrap(), params::ASPIRATION_WINDOW_SIZE)
    };

    loop {
        let Ok(eval) = negamax::negamax(game, window.alpha, window.beta, depth, 0, pv, ctx) else {
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
