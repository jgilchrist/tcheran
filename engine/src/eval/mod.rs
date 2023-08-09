use chess::{board::PlayerPieces, game::Game};

const PAWN_VALUE: i64 = 100;
const KNIGHT_VALUE: i64 = 300;
const BISHOP_VALUE: i64 = 300;
const ROOK_VALUE: i64 = 500;
const QUEEN_VALUE: i64 = 800;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Eval(i64);

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
pub fn eval(game: &Game) -> Eval {
    white_piece_value(game) + black_piece_value(game)
}

fn white_piece_value(game: &Game) -> Eval {
    count_piece_values(&game.board.white_pieces)
}

fn black_piece_value(game: &Game) -> Eval {
    -count_piece_values(&game.board.black_pieces)
}

fn count_piece_values(pieces: &PlayerPieces) -> Eval {
    Eval(
        i64::from(pieces.pawns.count()) * PAWN_VALUE
            + i64::from(pieces.knights.count()) * KNIGHT_VALUE
            + i64::from(pieces.bishops.count()) * BISHOP_VALUE
            + i64::from(pieces.rooks.count()) * ROOK_VALUE
            + i64::from(pieces.queens.count()) * QUEEN_VALUE,
    )
}
