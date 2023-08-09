mod material_diff;
mod piece_square_tables;

use chess::game::Game;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Eval(i32);

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

impl std::ops::Neg for Eval {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl std::fmt::Display for Eval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_value = self.0 as f32 / 100.0;
        write!(f, "{formatted_value}")
    }
}

#[allow(clippy::cast_possible_wrap)]
#[must_use]
pub fn eval(game: &Game) -> Eval {
    material_diff::material_diff(game)
        + piece_square_tables::piece_square_tables(game)
}
