use crate::chess::player::Player;
use crate::chess::square::Square;

pub struct HistoryTable([[[i32; Square::N]; Square::N]; Player::N]);

impl HistoryTable {
    pub const fn new() -> Self {
        todo!()
    }
}
