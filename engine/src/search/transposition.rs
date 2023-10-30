use crate::eval::Eval;
use crate::transposition_table::{TTOverwriteable, TranspositionTable};
use chess::moves::Move;
use chess::piece::PromotionPieceKind;
use chess::square::Square;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NodeBound {
    Exact,
    Upper,
    Lower,
}

#[derive(Debug, Clone)]
pub struct SearchTranspositionTableData {
    pub bound: NodeBound,
    pub eval: Eval,
    pub depth: u8,
    pub age: u8,
    pub best_move: Option<TTMove>,
}

#[derive(Debug, Clone)]
pub struct TTMove {
    start_square_idx: u8,
    end_square_idx: u8,
    promotion: Option<PromotionPieceKind>,
}

impl TTMove {
    pub fn from_move(mv: &Move) -> Self {
        Self {
            start_square_idx: mv.src.idx(),
            end_square_idx: mv.dst.idx(),
            promotion: mv.promotion,
        }
    }

    pub fn to_move(&self) -> Move {
        Move {
            src: Square::from_index(self.start_square_idx),
            dst: Square::from_index(self.end_square_idx),
            promotion: self.promotion,
        }
    }
}

impl TTOverwriteable for SearchTranspositionTableData {
    fn should_overwrite_with(&self, new: &Self) -> bool {
        // Always prioritise results from new searches
        if new.age != self.age {
            return true;
        }

        // If the new node is exact, always store it
        if new.bound == NodeBound::Exact {
            return true;
        }

        // Don't overwrite exact nodes
        self.bound != NodeBound::Exact
    }
}

pub type SearchTranspositionTable = TranspositionTable<SearchTranspositionTableData>;
