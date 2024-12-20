use std::fmt::Display;

use advent_of_code_2024::utils::{direction::Direction, grid::{Grid, GridPos}};

advent_of_code_2024::solution!(15);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum TilePartOne {
    Wall,
    Empty,
    Box,
    Robot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum TilePartTwo {
    Wall,
    Empty,
    BoxLeft,
    BoxRight,
    Robot,
}

const GPS_ROW_MULT: u64 = 100;

impl TilePartOne {
    pub fn from_part_two(t: TilePartTwo) -> Self {
        match t {
            TilePartTwo::Wall => Self::Wall,
            TilePartTwo::Empty => Self::Empty,
            TilePartTwo::BoxLeft => Self::Box,
            TilePartTwo::BoxRight => Self::Empty,
            TilePartTwo::Robot => Self::Robot,
        }
    }
}

impl TilePartTwo {
    pub fn from_part_one(t: TilePartOne) -> Self {
        match t {
            TilePartOne::Wall => Self::Wall,
            TilePartOne::Empty => Self::Empty,
            TilePartOne::Box => Self::BoxLeft,
            TilePartOne::Robot => Self::Robot,
        }
    }
}

impl Display for TilePartTwo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            TilePartTwo::Wall => '#',
            TilePartTwo::Empty => '.',
            TilePartTwo::BoxLeft => '[',
            TilePartTwo::BoxRight => ']',
            TilePartTwo::Robot => '@',
        };
        f.write_fmt(format_args!("{}", ch))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut warehouse, moves) = parse_input(input);
    let mut robot_pos = warehouse.index_of(|t| *t == TilePartOne::Robot).unwrap();
    // track robot by position, not grid value
    *warehouse.get_mut(&robot_pos).unwrap() = TilePartOne::Empty;

    for mv in moves {
        make_move_part_one(&mut warehouse, &mut robot_pos, mv);

    }
    Some(sum_box_coord(&warehouse))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (warehouse, moves) = parse_input(input);
    let warehouse = warehouse.grid_map(|_, t| TilePartTwo::from_part_one(t));
    let mut warehouse = stretch_warehouse(warehouse);
    let mut robot_pos = warehouse.index_of(|t| *t == TilePartTwo::Robot).unwrap();
    // track robot by position, not grid value
    *warehouse.get_mut(&robot_pos).unwrap() = TilePartTwo::Empty;

    for mv in moves {
        make_move_part_two(&mut warehouse, &mut robot_pos, mv);
    }

    let warehouse = warehouse.grid_map(|_, t| TilePartOne::from_part_two(t));
    Some(sum_box_coord(&warehouse))
}

fn parse_input(input: &str) -> (Grid<TilePartOne>, Vec<Direction>) {
    let mut lines = input.lines();
    let moves: Vec<Direction> = lines.next_back().unwrap().chars().map(ch_to_dir).collect();
    lines.next_back();
    let warehouse: Vec<Vec<TilePartOne>> = lines
        .map(|x| {
            x.chars().map(ch_to_tile).collect()
        })
        .collect();
    (Grid::from(warehouse), moves)
}

fn make_move_part_one(warehouse: &mut Grid<TilePartOne>, robot_pos: &mut GridPos, dir: Direction) {
    let new_pos = robot_pos.position_in_dir(dir);
    let target = *warehouse.get(&new_pos).unwrap();
    if target == TilePartOne::Wall {
        return;
    }
    if target == TilePartOne::Empty {
        *robot_pos = new_pos;
        return;
    }
    let scan = warehouse
        .scan_direction_until(
            &new_pos,
            dir.to_offset(),
            |_, t|  *t == TilePartOne::Empty
        )
        .expect(&format!("scanning {:?} from {} found nothing", dir, robot_pos));
    if scan.iter().position(|x| x.1 == TilePartOne::Wall).is_some() {
        return;
    }
    let box_end = scan.get(scan.len() - 1).unwrap().0.position_in_dir(dir);
    *warehouse.get_mut(&box_end).unwrap() = TilePartOne::Box;
    *warehouse.get_mut(&new_pos).unwrap() = TilePartOne::Empty;
    *robot_pos = new_pos;
}

fn make_move_part_two(warehouse: &mut Grid<TilePartTwo>, robot_pos: &mut GridPos, dir: Direction) {
    let new_pos = robot_pos.position_in_dir(dir);
    let target = *warehouse.get(&new_pos).unwrap();
    if target == TilePartTwo::Wall {
        return;
    }
    if target == TilePartTwo::Empty {
        *robot_pos = new_pos;
        return;
    }
    let target_box_pos = {
        if target == TilePartTwo::BoxLeft {
            new_pos
        } else {
            new_pos.position_in_dir(Direction::Left)
        }
    };

    if dir == Direction::Left || dir == Direction::Right {
        if can_push_box_left_right(warehouse, &target_box_pos, dir) {
            push_box_left_right(warehouse, &target_box_pos, dir);
            *robot_pos = new_pos;
        }
    } else if can_push_box_up_down(warehouse, &target_box_pos, dir) {
        push_box_up_down(warehouse, &target_box_pos, dir);
        *warehouse.get_mut(&target_box_pos).unwrap() = TilePartTwo::Empty;
        *warehouse
            .get_mut(&target_box_pos.position_in_dir(Direction::Right))
            .unwrap() = TilePartTwo::Empty;
        *robot_pos = new_pos;
    }
}

fn push_box_up_down(warehouse: &mut Grid<TilePartTwo>, box_pos: &GridPos, dir: Direction) {
    let left_new_pos = box_pos.position_in_dir(dir);
    let right_new_pos = box_pos.position_in_dir(Direction::Right).position_in_dir(dir);
    let left_current_contents = *warehouse.get(&left_new_pos).unwrap();
    let right_current_contents = *warehouse.get(&right_new_pos).unwrap();
    match (left_current_contents, right_current_contents) {
        (TilePartTwo::Empty, TilePartTwo::Empty) => (),
        (TilePartTwo::Empty, TilePartTwo::BoxLeft) => {
            push_box_up_down(warehouse, &right_new_pos, dir);
        },
        (TilePartTwo::BoxRight, TilePartTwo::Empty) => {
            push_box_up_down(warehouse, &left_new_pos.position_in_dir(Direction::Left), dir);
        },
        (TilePartTwo::BoxLeft, TilePartTwo::BoxRight) => {
            push_box_up_down(warehouse, &left_new_pos, dir);
        },
        (TilePartTwo::BoxRight, TilePartTwo::BoxLeft) => {
            push_box_up_down(warehouse, &right_new_pos, dir);
            push_box_up_down(warehouse, &left_new_pos.position_in_dir(Direction::Left), dir);
        }
        _ => unreachable!("x"),
    };
    *warehouse.get_mut(&left_new_pos).unwrap() = TilePartTwo::BoxLeft;
    *warehouse.get_mut(&right_new_pos).unwrap() = TilePartTwo::BoxRight;
    *warehouse.get_mut(box_pos).unwrap() = TilePartTwo::Empty;
    *warehouse.get_mut(&box_pos.position_in_dir(Direction::Right)).unwrap() = TilePartTwo::Empty;
}

fn push_box_left_right(warehouse: &mut Grid<TilePartTwo>, box_pos: &GridPos, dir: Direction) {
    let mut current_position = {
        if dir == Direction::Right {
            box_pos.position_in_dir(dir)
        } else {
            *box_pos
        }
    };
    let box_order = {
        if dir == Direction::Right {
            vec![TilePartTwo::BoxLeft, TilePartTwo::BoxRight]
        } else {
            vec![TilePartTwo::BoxRight, TilePartTwo::BoxLeft]
        }
    };
    for i in 0..warehouse.cols() {
        let was_empty = *warehouse.get(&current_position).unwrap() == TilePartTwo::Empty;
        *warehouse.get_mut(&current_position).unwrap() = box_order[i % 2];
        current_position = current_position.position_in_dir(dir);
        if was_empty {
            break;
        }
    }
    if dir == Direction::Right {
        *warehouse.get_mut(box_pos).unwrap() = TilePartTwo::Empty;
    } else {
        *warehouse.get_mut(&box_pos.position_in_dir(dir.opposite())).unwrap() = TilePartTwo::Empty;
    }
}

fn can_push_box_up_down(
    warehouse: &Grid<TilePartTwo>,
    box_pos: &GridPos,
    dir: Direction
) -> bool {
    let target_space_left = box_pos.position_in_dir(dir);
    let target_space_right = box_pos.position_in_dir(Direction::Right).position_in_dir(dir);
    if !(warehouse.is_valid_cell(&target_space_left)
        && warehouse.is_valid_cell(&target_space_right)
    ) {
        return false;
    }
    let target_tile_left = *warehouse.get(&target_space_left).unwrap();
    let target_tile_right = *warehouse.get(&target_space_right).unwrap();
    match (target_tile_left, target_tile_right) {
        (TilePartTwo::Wall, _) => false,
        (_, TilePartTwo::Wall) => false,
        (TilePartTwo::Empty, TilePartTwo::Empty) => true,
        (TilePartTwo::Empty, TilePartTwo::BoxLeft) => {
            can_push_box_up_down(warehouse, &target_space_right, dir)
        },
        (TilePartTwo::BoxRight, TilePartTwo::Empty) => {
            can_push_box_up_down(
                warehouse,
                &target_space_left.position_in_dir(Direction::Left),
                dir)
        },
        (TilePartTwo::BoxLeft, TilePartTwo::BoxRight) => {
            can_push_box_up_down(warehouse, &target_space_left, dir)
        },
        (TilePartTwo::BoxRight, TilePartTwo::BoxLeft) => {
            can_push_box_up_down(warehouse, &target_space_right, dir)
            && can_push_box_up_down(
                warehouse,
                &target_space_left.position_in_dir(Direction::Left),
                dir)
        }
        _ => unreachable!("checking to move box {:?} and found {:?} and {:?}",
            box_pos, target_tile_left, target_tile_right)
    }
}

fn can_push_box_left_right(
    warehouse: &Grid<TilePartTwo>,
    box_pos: &GridPos,
    dir: Direction
) -> bool {
    let new_space = {
        if dir == Direction::Left {
            box_pos.position_in_dir(dir)
        } else {
            box_pos.position_in_dir(dir).position_in_dir(dir)
        }
    };
    if !(warehouse.is_valid_cell(&new_space)
        && warehouse.is_valid_cell(&new_space.position_in_dir(Direction::Right))
    ) {
        return false;
    }

    match *warehouse.get(&new_space).unwrap() {
        TilePartTwo::Wall => false,
        TilePartTwo::Empty => true,
        TilePartTwo::BoxLeft => {
            if dir == Direction::Right {
                can_push_box_left_right(warehouse, &new_space, dir)
            } else {
                panic!("found a BoxLeft to the left {}", new_space);
            }
        },
        TilePartTwo::BoxRight => {
            if dir == Direction::Left {
                can_push_box_left_right(warehouse, &new_space.position_in_dir(Direction::Left), dir)
            } else {
                panic!("found a BoxRight to the right of box {}", new_space)
            }
        },
        TilePartTwo::Robot => unreachable!(),
    }
}

fn stretch_warehouse(warehouse: Grid<TilePartTwo>) -> Grid<TilePartTwo> {
    let mut new_warehouse = vec![];
    for r in 0..warehouse.rows() {
        let stretched: Vec<TilePartTwo> = warehouse.get_row(r).unwrap()
            .iter()
            .flat_map(stretch_tile)
            .collect();
        new_warehouse.push(stretched);
    }
    Grid::from(new_warehouse)
}

fn stretch_tile(t: &&TilePartTwo) -> Vec<TilePartTwo> {
    let t = **t;
    match t {
        TilePartTwo::Wall => vec![t, t],
        TilePartTwo::Empty => vec![t, t],
        TilePartTwo::BoxLeft => vec![t, TilePartTwo::BoxRight],
        TilePartTwo::Robot => vec![t, TilePartTwo::Empty],
        TilePartTwo::BoxRight => unreachable!("shouldn't be any boxrights yet")
    }
}

fn sum_box_coord(warehouse: &Grid<TilePartOne>) -> u64 {
    let mut coord_sum = 0;
    for (pos, tile) in warehouse.iter_by_rows() {
        if tile == TilePartOne::Box {
            coord_sum += GPS_ROW_MULT * pos.row as u64 + pos.col as u64;
        }
    }
    coord_sum
}

fn ch_to_tile(ch: char) -> TilePartOne {
    match ch {
        '#' => TilePartOne::Wall,
        'O' => TilePartOne::Box,
        '.' => TilePartOne::Empty,
        '@' => TilePartOne::Robot,
        _ => unreachable!("char {} isn't part of the map!", ch)
    }
}

fn ch_to_dir(ch: char) -> Direction {
    match ch {
        '^' => Direction::Up,
        'v' => Direction::Down,
        '<' => Direction::Left,
        '>' => Direction::Right,
        _ => unreachable!("char {} isn't a direction!", ch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const PART_ONE_TOY_EXAMPLE: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";
    const PART_TWO_TOY_EXAMPLE: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

    #[test]
    fn test_part_one_toy() {
        let result = part_one(PART_ONE_TOY_EXAMPLE);
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two_toy() {
        let result = part_two(PART_TWO_TOY_EXAMPLE);
        assert_eq!(result, Some(618));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(9021));
    }
}
