#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub const ALL: &'static [Direction; 8] = &[
        Direction::North,
        Direction::NorthEast,
        Direction::East,
        Direction::SouthEast,
        Direction::South,
        Direction::SouthWest,
        Direction::West,
        Direction::NorthWest,
    ];

    // TODO: Naming?
    pub const NON_DIAGONAL: &'static [Direction; 4] = &[
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    pub const DIAGONAL: &'static [Direction; 4] = &[
        Direction::NorthEast,
        Direction::SouthEast,
        Direction::SouthWest,
        Direction::NorthWest,
    ];
}
