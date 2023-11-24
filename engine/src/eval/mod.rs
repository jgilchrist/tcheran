mod material_diff;
mod piece_square_tables;

use chess::game::Game;

pub fn init() {
    piece_square_tables::init();
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Eval(pub i16);

impl std::ops::Add for Eval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Eval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Mul<i16> for Eval {
    type Output = Self;

    fn mul(self, rhs: i16) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl std::ops::Neg for Eval {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.saturating_neg())
    }
}

#[allow(clippy::cast_precision_loss)]
impl std::fmt::Display for Eval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_value = self.0 as f32 / 100.0;
        write!(f, "{formatted_value}")
    }
}

#[allow(clippy::cast_possible_wrap)]
#[must_use]
pub fn eval(game: &Game) -> Eval {
    material_diff::material_diff(game) + piece_square_tables::piece_square_tables(game)
}

#[derive(Debug)]
pub struct EvalComponents {
    pub eval: Eval,
    pub material: Eval,
    pub piece_square_tables_white: Eval,
    pub piece_square_tables_black: Eval,
    pub piece_square_tables: Eval,
}

#[must_use]
pub fn eval_components(game: &Game) -> EvalComponents {
    let eval = eval(game);
    let material = material_diff::material_diff(game);
    let piece_square_tables_white = piece_square_tables::piece_square_tables_white(game);
    let piece_square_tables_black = piece_square_tables::piece_square_tables_black(game);
    let piece_square_tables = piece_square_tables::piece_square_tables(game);

    EvalComponents {
        eval,
        material,
        piece_square_tables_white,
        piece_square_tables_black,
        piece_square_tables,
    }
}
