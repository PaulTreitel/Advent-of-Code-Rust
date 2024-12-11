advent_of_code_2022::solution!(22);

use std::{cmp::max, collections::{HashMap, HashSet}, fmt::Display};

use advent_of_code_2022::utils::direction::Direction;

type Part2Edge = (i32, i32, Option<(Direction, Direction)>);

struct ContextPart1 {
    edges: HashMap<(i32, i32), HashSet<(i32, i32)>>,
    start: (i32, i32),
    moves: Vec<i32>,
    turns: Vec<char>,
}

struct ContextPart2 {
    edges: HashMap<(i32, i32), HashSet<Part2Edge>>,
    start: (i32, i32),
    moves: Vec<i32>,
    turns: Vec<char>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Face {
    top_left: (i32, i32),
    bottom_right: (i32, i32)
}

impl Display for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Face{:?}", self.top_left))
    }
}

impl Face {
    pub fn get_corners(&self) -> Vec<(i32, i32)> {
        vec![
            self.top_left,
            self.bottom_right,
            (self.top_left.0, self.bottom_right.1),
            (self.bottom_right.0, self.top_left.1)
        ]
    }

    pub fn map_adjacent(&self, other: &Face) -> bool {
        let mut corners = HashSet::new();
        corners.extend(self.get_corners());
        corners.extend(other.get_corners());
        corners.len() < 8
    }

    pub fn orthogonal_map_adjacent(&self, other: &Face) -> bool {
        let mut corners = HashSet::new();
        corners.extend(self.get_corners());
        corners.extend(other.get_corners());
        corners.len() == 6
    }

    pub fn direction_to(&self, other: &Face) -> Option<Direction> {
        let same_row = self.top_left.0 == other.top_left.0;
        let same_col = self.top_left.1 == other.top_left.1;
        let left = self.top_left.1 == other.bottom_right.1;
        let right = self.bottom_right.1 == other.top_left.1;
        let above = self.top_left.0 == other.bottom_right.0;
        let below = self.bottom_right.0 == other.top_left.0;

        if above && same_col {
            Some(Direction::Up)
        } else if below && same_col {
            Some(Direction::Down)
        } else if left && same_row {
            Some(Direction::Left)
        } else if right && same_row {
            Some(Direction::Right)
        } else if above && left {
            Some(Direction::UpLeft)
        } else if above && right {
            Some(Direction::UpRight)
        } else if below && left {
            Some(Direction::DownLeft)
        } else if below && right {
            Some(Direction::DownRight)
        } else {
            None
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let ctx = get_graph(input);
    let mut curr_pos = ctx.start;
    let mut curr_dir = Direction::Right;

    for i in 0..ctx.moves.len() {
        make_move(&ctx, &mut curr_pos, &curr_dir, ctx.moves.get(i).unwrap());
        if i < ctx.turns.len() {
            new_direction(&mut curr_dir, *ctx.turns.get(i).unwrap());
        }
    }
    Some((curr_pos.0 + 1) * 1000 + (curr_pos.1 + 1) * 4 + get_dir_val(&curr_dir))
}

pub fn part_two(input: &str) -> Option<i32> {
    let ctx = get_graph_part2(input);
    let mut curr_pos = ctx.start;
    let mut curr_dir = Direction::Right;
    println!("now at {:?}, facing {:?}", curr_pos, curr_dir);

    for i in 0..ctx.moves.len() {
        make_move_part2(&ctx, &mut curr_pos, &mut curr_dir, ctx.moves.get(i).unwrap());
        if i < ctx.turns.len() {
            new_direction(&mut curr_dir, *ctx.turns.get(i).unwrap());
        }
        println!("now at {:?}, facing {:?}", curr_pos, curr_dir);
    }
    Some((curr_pos.0 + 1) * 1000 + (curr_pos.1 + 1) * 4 + get_dir_val(&curr_dir))
    // None
}

fn make_move_part2(
    ctx: &ContextPart2,
    pos: &mut (i32, i32),
    dir: &mut Direction,
    mv: &i32
) {
    println!("sitting at {:?} facing {:?}, moving {} tiles", pos, dir, mv);
    for _ in 0..*mv {
        let neighbors = ctx.edges.get(pos).unwrap();
        for (r, c, side_dir) in neighbors {
            match side_dir {
                Some((appear_dir, move_dir)) => {
                    if move_dir == dir {
                        println!("wrapping from {:?} to ({},{}), now facing {:?} after moving {:?}",
                            pos, r, c, appear_dir, move_dir
                        );
                        *pos = (*r, *c);
                        *dir = *appear_dir;
                        break;
                    }
                },
                None => {
                    if make_normal_move(r, c, pos, dir) {
                        break;
                    }
                },
            }
        }
    }
}

fn make_normal_move(r: &i32, c: &i32, pos: &mut (i32, i32), dir: &Direction) -> bool {
    let did_match = match *dir {
        Direction::Left => *r == pos.0 && (*c == pos.1 - 1 || *c > pos.1 + 1),
        Direction::Right => *r == pos.0 && (*c == pos.1 + 1 || *c < pos.1 - 1),
        Direction::Up => (*r == pos.0 - 1 || *r > pos.0 + 1) && *c == pos.1,
        Direction::Down => (*r == pos.0 + 1 || *r < pos.0 - 1) && *c == pos.1,
        _ => unreachable!()
    };
    if did_match {
        *pos = (*r, *c);
    }
    did_match
}

fn get_graph_part2(input: &str) -> ContextPart2 {
    let mut points = HashSet::new();
    let mut start = (0, 0);

    let mut input: Vec<&str> = input.lines().collect();
    let (moves, turns) = get_instructions(input.pop().unwrap());
    input.pop().unwrap();
    let char_vecs: Vec<Vec<char>> = input.iter()
        .map(|l| l.chars().collect())
        .collect();

    get_points_start(&char_vecs, &mut points, &mut start);
    let mut edges = get_normal_edges_part2(&points);
    let side_len = get_side_len(&char_vecs);
    let faces = get_faces(&char_vecs, side_len);
    let face_edges = get_face_edges(&char_vecs, &faces);
    add_face_wraparounds(&mut edges, &face_edges, &points, side_len);

    ContextPart2 { edges, start, moves, turns }
}

fn add_face_wraparounds(
    edges: &mut HashMap<(i32, i32), HashSet<Part2Edge>>,
    face_edges: &HashMap<(Face, Face), Direction>,
    points: &HashSet<(i32, i32)>,
    side_len: i32
) {
    let mut viited = Vec::new();
    for ((from, to), dir1) in face_edges {
        if viited.contains(&(*from, *to)) || from.orthogonal_map_adjacent(to) {
            continue;
        }
        let opp_edge = (*to, *from);
        viited.push((*from, *to));
        viited.push(opp_edge);
        let dir2 = face_edges.get(&opp_edge).unwrap();
        let mut from_pts = points_from_face_edge(dir2, from, side_len, points);
        // TODO when do we need to reverse this?
        let mut to_pts = points_from_face_edge(dir1, to, side_len, points);
        if *dir1 == Direction::Right && dir2.opposite() == Direction::Down {
            to_pts.reverse();
        }
        if *dir2 == Direction::Right && dir1.opposite() == Direction::Down {
            from_pts.reverse();
        }

        if *dir2 == Direction::Left && dir1.opposite() == Direction::Up {
            from_pts.reverse();
        }

        for (pt_1, pt_2) in from_pts.iter().zip(&to_pts) {
            match edges.get_mut(pt_1) {
                Some(x) => {
                    // appear dir, move dir
                    x.insert((pt_2.0, pt_2.1, Some((dir1.opposite(), *dir2))));
                },
                None => (),
            }
            match edges.get_mut(pt_2) {
                Some(x) => {
                    x.insert((pt_1.0, pt_1.1, Some((dir2.opposite(), *dir1))));
                },
                None => (),
            }
        }
    }
}

fn points_from_face_edge(
    dir: &Direction,
    from: &Face,
    side_len: i32,
    points: &HashSet<(i32, i32)>
) -> Vec<(i32, i32)> {
    match dir {
        Direction::Up => {
            (0..side_len)
                .map(|col| (from.top_left.0, from.top_left.1 + col))
                .filter(|x| points.contains(x))
                .collect::<Vec<(i32, i32)>>()

        },
        Direction::Down => {
            (0..side_len)
                .map(|col| (from.bottom_right.0 - 1, from.top_left.1 + col))
                .filter(|x| points.contains(x))
                .collect::<Vec<(i32, i32)>>()
        },
        Direction::Left => {
            (0..side_len)
                .map(|row| (from.top_left.0 + row - 1, from.top_left.1))
                .filter(|x| points.contains(x))
                .collect::<Vec<(i32, i32)>>()
        },
        Direction::Right => {
            (0..side_len)
                .map(|row| (from.top_left.0 + row, from.bottom_right.1 - 1))
                .filter(|x| points.contains(x))
                .collect::<Vec<(i32, i32)>>()
        },
        _ => unreachable!(),
    }
}

fn get_faces(char_vecs: &Vec<Vec<char>>, side_len: i32) -> Vec<Face> {
    let width = char_vecs.iter().map(|x| x.len()).max().unwrap();
    let mut faces = vec![];
    for row_idx in (0..char_vecs.len()).step_by(side_len as usize) {
        for col_idx in (0..width).step_by(side_len as usize) {
            if char_vecs.get(row_idx).unwrap().len() <= col_idx {
                continue;
            }
            if *char_vecs.get(row_idx).unwrap().get(col_idx).unwrap() != ' ' {
                faces.push(Face {
                    top_left: (row_idx as i32, col_idx as i32),
                    bottom_right: (row_idx as i32 + side_len, col_idx as i32 + side_len),
                });
            }
        }
    }
    faces
}

fn get_face_edges(char_vecs: &Vec<Vec<char>>, faces: &Vec<Face>) -> HashMap<(Face, Face), Direction> {
    let side_len = get_side_len(char_vecs);
    let mut face_edges = HashMap::new();
    let mut found_edges: HashMap<Face, Vec<(&Face, Direction)>> = HashMap::new();

    for f1 in faces {
        for f2 in faces {
            if f1 == f2 {
                continue;
            }
            if f1.map_adjacent(&f2) {
                let new1 = (f2, f1.direction_to(&f2).unwrap());
                found_edges
                    .entry(*f1)
                    .and_modify(|x| x.push(new1))
                    .or_insert(vec![new1]);
            }
        }
    }
    add_adjacent_face_edges(&mut face_edges, &found_edges, side_len);

    // If we connect every face with 1 or 2 edges to every 2nd order neighbor
    // with less than 4 neighbors, we'll make all the right connections. Then we
    // just do the same but with faces with 3 edges.

    for face in faces {
        let first_order_neighbors = found_edges.get(face).unwrap().clone();
        if first_order_neighbors.len() > 2 {
            continue;
        }
        add_edges_to_secondary_neighbors(&mut found_edges, &mut face_edges, &first_order_neighbors, face);
    }
    for _ in 0..2 {
        for face in faces {
            let first_order_neighbors = found_edges.get(face).unwrap().clone();
            if first_order_neighbors.len() == 4 {
                continue;
            }
            add_edges_to_secondary_neighbors(&mut found_edges, &mut face_edges, &first_order_neighbors, face);
        }
    }
    face_edges
}

fn add_edges_to_secondary_neighbors<'a>(
    found_edges: &mut HashMap<Face, Vec<(&'a Face, Direction)>>,
    face_edges: &mut HashMap<(Face, Face), Direction>,
    first_order_neighbors: &Vec<(&Face, Direction)>,
    face: &'a Face,
) {
    let mut edges_to_add = Vec::new();
    for (first_neighbor, first_dir) in first_order_neighbors {
        let second_order_neighbors = found_edges.get(first_neighbor).unwrap();
        for (second_neighbor, second_dir) in second_order_neighbors {
            if face_edges.contains_key(&(*face, **second_neighbor)) {
                continue;
            }
            if found_edges.get(second_neighbor).unwrap().len() == 4 {
                continue;
            }
            add_final_face_edge(
                &mut edges_to_add,
                face,
                *second_neighbor,
                (first_dir, second_dir)
            );
        }
    }
    for e in edges_to_add {
        found_edges.get_mut(&e.0).unwrap().push((e.1, e.2));
        face_edges.insert((e.0, *e.1), e.2);
    }
}

fn add_final_face_edge<'a>(
    to_add: &mut Vec<(Face, &'a Face, Direction)>,
    start: &'a Face,
    end: &'a Face,
    directions: (&Direction, &Direction)
) {
    let dir = match directions.0 {
        Direction::Up => {
            match directions.1 {
                Direction::UpLeft => Direction::Up, // top
                Direction::UpRight => Direction::Left, // left
                _ => return,
            }
        },
        Direction::Down => match directions.1 {
            Direction::Right => Direction::Right,
            Direction::Left => Direction::Left,
            _ => return,
        },
        Direction::Left => match directions.1 {
            _ => return,
        },
        Direction::Right => match directions.1 {
            Direction::UpRight => Direction::Up,
            _ => return,
        },
        Direction::UpLeft => match directions.1 {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Down,
            _ => return,
        },
        Direction::UpRight => match directions.1 {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Down,
            _ => return,
        },
        Direction::DownLeft => match directions.1 {
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Right,
            _ => return,
        },
        Direction::DownRight => match directions.1 {
            Direction::Down => Direction::Left,
            _ => return,
        },
    };
    to_add.push((*start, end, dir));
    to_add.push((*end, start, dir));
}

fn add_adjacent_face_edges(
    face_edges: &mut HashMap<(Face, Face), Direction>,
    found_edges: &HashMap<Face, Vec<(&Face, Direction)>>,
    side_len: i32
) {
    for (start_face, edges) in found_edges {
        let left_face = Face {
            top_left: (start_face.top_left.0, start_face.top_left.1 - side_len),
            bottom_right: (start_face.bottom_right.0, start_face.bottom_right.1 - side_len)
        };
        let right_face = Face {
            top_left: (start_face.top_left.0, start_face.top_left.1 + side_len),
            bottom_right: (start_face.bottom_right.0, start_face.bottom_right.1 + side_len)
        };
        for e in edges {
            let arrival_direction = match e.1 {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
                Direction::UpLeft => {
                    if found_edges.contains_key(&left_face) {
                        Direction::Right
                    } else {
                        Direction::Down
                    }
                },
                Direction::UpRight => {
                    if found_edges.contains_key(&right_face) {
                        Direction::Left
                    } else {
                        Direction::Up
                    }
                },
                Direction::DownLeft => {
                    if found_edges.contains_key(&left_face) {
                        Direction::Right
                    } else {
                        Direction::Up
                    }
                },
                Direction::DownRight => {
                    if found_edges.contains_key(&right_face) {
                        Direction::Left
                    } else {
                        Direction::Up
                    }
                },
            };
            face_edges.insert((*start_face, *e.0), arrival_direction);
        }
    }
}

fn get_side_len(char_vecs: &Vec<Vec<char>>) -> i32 {
    let square_ct = char_vecs
        .iter()
        .flatten()
        .filter(|x| **x != ' ')
        .count();
    ((square_ct / 6) as f64).sqrt() as i32
}

fn get_normal_edges_part2(points: &HashSet<(i32, i32)>
) -> HashMap<(i32, i32), HashSet<Part2Edge>> {
    let mut normal_edges = HashMap::new();
    add_normal_edges(&mut normal_edges, &points);
    let edges = normal_edges
        .iter()
        .map(|(k, v)|
            (   *k,
                v.iter()
                    .map(|x| (x.0, x.1, None))
                    .collect::<HashSet<Part2Edge>>()
            )
        )
        .collect();
    edges
}

fn get_dir_val(dir: &Direction) -> i32 {
    match dir {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
        _ => unreachable!()
    }
}

fn make_move(
    ctx: &ContextPart1,
    pos: &mut (i32, i32),
    dir: &Direction,
    mv: &i32
) {
    for _ in 0..*mv {
        let neighbors = ctx.edges.get(pos).unwrap();
        let mut moved = false;
        for (r, c) in neighbors {
            if make_normal_move(r, c, pos, dir) {
                moved = true;
                break;
            }
        }
        if !moved {
            break;
        }
    }
}

fn new_direction(dir: &mut Direction, turn: char) {
    if turn == 'L' {
        dir.turn_left();
    } else if turn == 'R' {
        dir.turn_right();
    }
}

fn get_graph(input: &str) -> ContextPart1 {
    let mut points = HashSet::new();
    let mut edges = HashMap::new();
    let mut start = (0, 0);

    let mut input: Vec<&str> = input.lines().collect();
    let (moves, turns) = get_instructions(input.pop().unwrap());
    input.pop().unwrap();
    let char_vecs: Vec<Vec<char>> = input.iter()
        .map(|l| l.chars().collect())
        .collect();

    get_points_start(&char_vecs, &mut points, &mut start);
    add_normal_edges(&mut edges, &points);
    add_wraparound_edges(&char_vecs, &mut edges);
    ContextPart1 { edges, start, moves, turns }
}

fn get_instructions(input: &str) -> (Vec<i32>, Vec<char>) {
    let input = input.to_string();
    let turns = input.split(|x: char| x.is_numeric())
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().next().unwrap())
        .collect();
    let moves: Vec<i32> = input
        .split(|p: char| p.is_alphabetic())
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    (moves, turns)
}

fn add_wraparound_edges(
    char_vecs: &Vec<Vec<char>>,
    edges: &mut HashMap<(i32, i32), HashSet<(i32, i32)>>
) {
    let mut max_width = 0;
    for row_idx in 0..char_vecs.len() {
        let row = char_vecs.get(row_idx).unwrap();
        max_width = max(max_width, row.len());
        if *row.last().unwrap() == '#' {
            continue;
        }
        for col_idx in 0..row.len() {
            let cell = row.get(col_idx).unwrap();
            if *cell == '#' {
                break;
            } else if *cell == '.' {
                edges.get_mut(&(row_idx as i32, col_idx as i32)).unwrap()
                    .insert((row_idx as i32, row.len() as i32 - 1));
                edges.get_mut(&(row_idx as i32, row.len() as i32 - 1)).unwrap()
                    .insert((row_idx as i32, col_idx as i32));
                break;
            }
        }
    }

    for col_idx in 0..max_width {
        let mut col_start = (i32::MAX, i32::MAX);
        let mut col_end = (i32::MAX, i32::MAX);
        for row_idx in 0..char_vecs.len() {
            let row = char_vecs.get(row_idx).unwrap();
            if row.len() <= col_idx {
                continue;
            }
            let val = row.get(col_idx).unwrap();
            if *val != ' ' {
                if col_start == (i32::MAX, i32::MAX) {
                    if *val == '#' {
                        break;
                    }
                    col_start = (row_idx as i32, col_idx as i32);
                }
                col_end = (row_idx as i32, col_idx as i32);
            }
        }
        if col_end == (i32::MAX, i32::MAX) || col_start == (i32::MAX, i32::MAX) {
            continue;
        }
        let endpoint = char_vecs.get(col_end.0 as usize).unwrap()
            .get(col_end.1 as usize).unwrap();
        if *endpoint == '.' {
            edges.get_mut(&col_start).unwrap().insert(col_end);
            edges.get_mut(&col_end).unwrap().insert(col_start);
        }
    }
}

fn add_normal_edges(
    edges: &mut HashMap<(i32, i32), HashSet<(i32, i32)>>,
    points: &HashSet<(i32, i32)>
) {
    for (x, y) in points {
        let mut neighbors = HashSet::new();
        if points.contains(&(x + 1, *y)) {
            neighbors.insert((x + 1, *y));
        }
        if points.contains(&(x - 1, *y)) {
            neighbors.insert((x - 1, *y));
        }
        if points.contains(&(*x, y + 1)) {
            neighbors.insert((*x, y + 1));
        }
        if points.contains(&(*x, y - 1)) {
            neighbors.insert((*x, y - 1));
        }
        edges.insert((*x, *y), neighbors);
    }
}

fn get_points_start(
    char_vecs: &Vec<Vec<char>>,
    points: &mut HashSet<(i32, i32)>,
    start: &mut (i32, i32)
) {
    for row_idx in 0..char_vecs.len() {
        let row = char_vecs.get(row_idx).unwrap();
        for col_idx in 0..row.len() {
            if *row.get(col_idx).unwrap() == '.' {
                if points.is_empty() {
                    *start = (row_idx as i32, col_idx as i32);
                }
                points.insert((row_idx as i32, col_idx as i32));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(5031));
    }
}

