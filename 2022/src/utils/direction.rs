
pub const CARDINAL_DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];
pub const DIAG_DIRECTIONS: [Direction; 4] = [
    Direction::UpLeft,
    Direction::UpRight,
    Direction::DownLeft,
    Direction::DownRight,
];

pub const ALL_DIRECTIONS: [Direction; 8] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
    Direction::UpLeft,
    Direction::UpRight,
    Direction::DownLeft,
    Direction::DownRight,
];

/* When scanning for sequences in a grid, one may want to scan just right/down
 * orthogonally and just down-left/down-right diagonally to avoid getting
 * duplicates from scanning the same sequence from above and below
 */
pub const CARDINAL_DOWN_RIGHT_DIRECTIONS: [Direction; 2] = [Direction::Down, Direction::Right];
pub const DIAG_DOWN_DIRECTIONS: [Direction; 2] = [Direction::DownLeft, Direction::DownLeft];
pub const ALL_DOWN_DIRECTION: [Direction; 4] = [
    Direction::Down,
    Direction::Right,
    Direction::DownLeft,
    Direction::DownRight,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn from_offset(offsets: (i32, i32)) -> Option<Self> {
        let (row, col) = offsets;
        let out = match row.cmp(&0) {
            std::cmp::Ordering::Less => match col.cmp(&0) {
                std::cmp::Ordering::Less => Self::UpLeft,
                std::cmp::Ordering::Equal => Self::Up,
                std::cmp::Ordering::Greater => Self::UpRight,
            },
            std::cmp::Ordering::Equal => match col.cmp(&0) {
                std::cmp::Ordering::Less => Self::Left,
                std::cmp::Ordering::Equal => return None,
                std::cmp::Ordering::Greater => Self::Right,
            },
            std::cmp::Ordering::Greater => match col.cmp(&0) {
                std::cmp::Ordering::Less => Self::DownLeft,
                std::cmp::Ordering::Equal => Self::Down,
                std::cmp::Ordering::Greater => Self::DownRight,
            },
        };
        Some(out)
    }

    pub fn to_offset(&self) -> (i32, i32) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
            Self::UpLeft => (-1, -1),
            Self::UpRight => (-1, 1),
            Self::DownLeft => (1, -1),
            Self::DownRight => (1, 1),
        }
    }

    pub fn turn_right(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::UpLeft => Self::UpRight,
            Self::UpRight => Self::DownRight,
            Self::DownRight => Self::DownLeft,
            Self::DownLeft => Self::UpLeft,
        };
    }

    pub fn turn_left(&mut self) {
        *self = match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
            Self::UpLeft => Self::DownLeft,
            Self::DownLeft => Self::DownRight,
            Self::DownRight => Self::UpRight,
            Self::UpRight => Self::UpLeft,
        };
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::UpLeft => Self::DownRight,
            Self::UpRight => Self::DownLeft,
            Self::DownLeft => Self::UpRight,
            Self::DownRight => Self::UpLeft,
        }
    }
}
