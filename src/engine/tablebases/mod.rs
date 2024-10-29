use crate::chess::game::Game;
use crate::chess::moves::{Move, MoveListExt};
use crate::chess::piece::PromotionPieceKind;
use crate::chess::square::Square;
use std::path::Path;

pub enum Wdl {
    Win,
    Draw,
    Loss,
}

pub struct Tablebase {
    shakmaty_tb: shakmaty_syzygy::Tablebase<shakmaty::Chess>,
    is_enabled: bool,
}

impl Tablebase {
    pub fn new() -> Self {
        Self {
            shakmaty_tb: shakmaty_syzygy::Tablebase::new(),
            is_enabled: false,
        }
    }

    #[expect(unused)]
    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    #[expect(unused)]
    pub fn n_men(&self) -> usize {
        self.shakmaty_tb.max_pieces()
    }

    pub fn set_paths(&mut self, path: &str) -> Result<(), ()> {
        let path = path.to_string();
        let separator = if cfg!(windows) { ';' } else { ':' };

        let paths = path.split(separator).map(|p| Path::new(p));

        for p in paths {
            self.shakmaty_tb.add_directory(p).expect(&format!(
                "Invalid tablebase path: {}",
                p.to_str().unwrap_or_default()
            ));
        }

        self.is_enabled = true;
        Ok(())
    }

    #[expect(unused)]
    pub fn wdl(&self, game: &Game) -> Option<Wdl> {
        if !self.is_enabled {
            return None;
        }

        let shakmaty_pos = Self::pos_to_shakmaty_pos(game);

        use shakmaty_syzygy::Wdl::*;
        match self.shakmaty_tb.probe_wdl(&shakmaty_pos) {
            Ok(m) => m.unambiguous().map(|wdl| match wdl {
                Loss => Wdl::Loss,
                BlessedLoss | Draw | CursedWin => Wdl::Draw,
                Win => Wdl::Win,
            }),
            Err(_) => None,
        }
    }

    pub fn best_move(&self, game: &Game) -> Option<Move> {
        if !self.is_enabled {
            return None;
        }

        let shakmaty_pos = Self::pos_to_shakmaty_pos(game);

        match self.shakmaty_tb.best_move(&shakmaty_pos) {
            Ok(m) => m.map(|(shakmaty_mv, _)| Self::shakmaty_mv_to_move(game, &shakmaty_mv)),
            Err(_) => None,
        }
    }

    fn pos_to_shakmaty_pos(game: &Game) -> shakmaty::Chess {
        game.to_fen()
            .parse::<shakmaty::fen::Fen>()
            .unwrap()
            .into_position(shakmaty::CastlingMode::Standard)
            .unwrap()
    }

    fn shakmaty_mv_to_move(game: &Game, shakmaty_mv: &shakmaty::Move) -> Move {
        use shakmaty::Role;

        let src = Square::from_index(shakmaty_mv.from().unwrap() as u8);
        let dst = Square::from_index(shakmaty_mv.to() as u8);
        let promotion = shakmaty_mv.promotion().map(|p| match p {
            Role::Knight => PromotionPieceKind::Knight,
            Role::Bishop => PromotionPieceKind::Bishop,
            Role::Rook => PromotionPieceKind::Rook,
            Role::Queen => PromotionPieceKind::Queen,
            Role::King | Role::Pawn => unreachable!(),
        });

        game.moves().expect_matching(src, dst, promotion)
    }
}
