use std::{
    cmp::{min, Ordering},
    fmt::{Debug, Display},
};

use super::direction::Direction;
pub trait GridCell: Clone + PartialEq + Ord + Debug {}
impl<T> GridCell for T where T: Clone + PartialEq + Ord + Debug {}

#[derive(Debug, Clone)]
pub struct GridIterator<GridCell> {
    grid_idxs: Vec<(GridPos, GridCell)>,
}

impl<GridCell> Iterator for GridIterator<GridCell> {
    type Item = (GridPos, GridCell);

    fn next(&mut self) -> Option<Self::Item> {
        self.grid_idxs.pop()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GridPos {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Grid<T: GridCell> {
    grid: Vec<Vec<T>>,
    rows: usize,
    cols: usize,
}

impl GridPos {
    pub fn new(row: usize, col: usize) -> Self {
        GridPos { row, col }
    }

    pub fn move_in_dir(&mut self, dir: Direction) -> &mut Self {
        self.row = (self.row as i32 + dir.to_offset().0) as usize;
        self.col = (self.col as i32 + dir.to_offset().1) as usize;
        self
    }

    pub fn get_orthogonal_neighbors(&self) -> Vec<GridPos> {
        let mut left = *self;
        let mut right = *self;
        let mut up = *self;
        let mut down = *self;
        left.move_in_dir(Direction::Left);
        right.move_in_dir(Direction::Right);
        up.move_in_dir(Direction::Up);
        down.move_in_dir(Direction::Down);

        vec![left, right, up, down]
    }
}

impl Display for GridPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.row, self.col))
    }
}

impl<T: GridCell> Grid<T> {
    pub fn from(grid: Vec<Vec<T>>) -> Self {
        let rows = grid.len();
        let cols = grid.first().unwrap().len();
        if rows == 0 {
            panic!("Grid is empty!");
        } else if !Grid::rows_same_len(&grid) {
            panic!("Grid is not a rectangle!");
        } else if cols == 0 {
            panic!("Grid is empty!")
        }
        Grid { grid, rows, cols }
    }

    pub fn new(rows: usize, cols: usize, default: T) -> Self {
        let mut grid = Vec::with_capacity(rows);
        for _ in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for _ in 0..cols {
                row.push(default.clone());
            }
            grid.push(row);
        }
        Grid { grid, rows, cols }
    }

    fn rows_same_len(grid: &Vec<Vec<T>>) -> bool {
        let row_len = grid.first().unwrap().len();
        for row in grid {
            if row.len() != row_len {
                return false;
            }
        }
        true
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn get(&self, pos: &GridPos) -> Option<&T> {
        self.grid.get(pos.row)?.get(pos.col)
    }

    pub fn get_mut(&mut self, pos: &GridPos) -> Option<&mut T> {
        self.grid.get_mut(pos.row)?.get_mut(pos.col)
    }

    pub fn get_row(&self, row: usize) -> Option<Vec<&T>> {
        if row >= self.rows {
            return None;
        }
        let row = self.grid.get(row)?.iter().collect::<Vec<&T>>();
        Some(row)
    }

    pub fn get_col(&self, col: usize) -> Option<Vec<&T>> {
        if col >= self.cols {
            return None;
        }
        let col: Vec<Option<&T>> = self.grid.iter().map(|row| row.get(col)).collect();
        let mut new_col = Vec::new();
        for item in col {
            match item {
                Some(x) => new_col.push(x),
                None => return None,
            }
        }
        Some(new_col)
    }

    pub fn index_of(&self, eq: &T) -> Option<GridPos> {
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.grid.get(r)?.get(c)?.eq(eq) {
                    return Some(GridPos::new(r, c));
                }
            }
        }
        None
    }

    pub fn count(&self, cmp: impl Fn(&T) -> bool) -> u64 {
        let mut count = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if cmp(self.grid.get(r).unwrap().get(c).unwrap()) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn is_valid_cell(&self, pos: &GridPos) -> bool {
        pos.row < self.rows && pos.col < self.cols
    }

    pub fn grid_clone(&self) -> Vec<Vec<T>> {
        self.grid.clone()
    }

    pub fn valid_directional_scan(
        &self,
        pos: &GridPos,
        offset: (i32, i32),
        scan_len: usize,
    ) -> bool {
        let row_min_fails = offset.0 < 0 && pos.row < scan_len;
        let row_max_fails = offset.0 > 0 && pos.row >= self.rows - scan_len;
        let col_min_fails = offset.1 < 0 && pos.col < scan_len;
        let col_max_fails = offset.1 > 0 && pos.col >= self.cols - scan_len;
        !(row_max_fails || row_min_fails || col_max_fails || col_min_fails)
    }

    pub fn scan_direction(
        &self,
        pos: &GridPos,
        offset: (i32, i32),
        scan_len: usize,
    ) -> Option<Vec<(GridPos, T)>> {
        if offset.0 == 0 && offset.1 == 0 {
            return None;
        }
        if !self.valid_directional_scan(pos, offset, scan_len - 1) {
            return None;
        }
        let mut scan_result = Vec::new();
        for i in 0..scan_len as i32 {
            let target_pos = GridPos::new(
                (pos.row as i32 + offset.0 * i) as usize,
                (pos.col as i32 + offset.1 * i) as usize,
            );
            let val = self.get(&target_pos)?.clone();
            scan_result.push((target_pos, val));
        }
        Some(scan_result)
    }

    fn max_scan_iterations(&self, pos: &GridPos, offset: (i32, i32)) -> i32 {
        let until_rows_end = match offset.0.cmp(&0) {
            Ordering::Less => pos.row as i32 / offset.0.abs(),
            Ordering::Equal => self.cols as i32,
            Ordering::Greater => (self.rows as i32 - pos.row as i32) / offset.0,
        };
        let until_cols_end = match offset.1.cmp(&0) {
            Ordering::Less => pos.col as i32 / offset.1.abs(),
            Ordering::Equal => self.rows as i32,
            Ordering::Greater => (self.cols as i32 - pos.col as i32) / offset.1,
        };
        min(until_rows_end, until_cols_end) + 1
    }

    pub fn scan_direction_until(
        &self,
        pos: &GridPos,
        offset: (i32, i32),
        stop_condition: impl Fn(GridPos, &T) -> bool,
    ) -> Option<Vec<(GridPos, T)>> {
        if offset.0 == 0 && offset.1 == 0 {
            return None;
        }
        let max_scan_len = self.max_scan_iterations(pos, offset);
        if max_scan_len <= 0 {
            return None;
        }
        let mut scan_result = Vec::new();
        for i in 0..max_scan_len {
            let target_pos = GridPos::new(
                (pos.row as i32 + offset.0 * i) as usize,
                (pos.col as i32 + offset.1 * i) as usize,
            );
            match self.get(&target_pos) {
                Some(c) => {
                    if stop_condition(target_pos, c) {
                        break;
                    }
                    scan_result.push((target_pos, c.clone()))
                }
                None => return Some(scan_result),
            }
        }
        Some(scan_result)
    }

    fn get_iterator_grid(&self) -> Vec<(GridPos, T)> {
        let mut new_grid = Vec::new();
        for row_idx in 0..self.rows {
            let row = self.grid.get(row_idx).unwrap();
            let new_row: Vec<(GridPos, T)> = row
                .iter()
                .enumerate()
                .map(|(col_idx, v)| (GridPos::new(row_idx, col_idx), v.clone()))
                .collect();
            new_grid.extend_from_slice(&new_row);
        }
        new_grid
    }

    fn iterate_by(&self, rows: bool) -> GridIterator<T> {
        let mut cells = self.get_iterator_grid();
        if rows {
            cells.sort();
        } else {
            cells.sort_by(|(x, _), (y, _)| Grid::<T>::column_cell_ordering(x, y));
        }
        cells.reverse();
        let x = GridIterator { grid_idxs: cells };
        x.into_iter()
    }

    pub fn iter_by_rows(&self) -> GridIterator<T> {
        self.iterate_by(true)
    }

    pub fn iter_by_cols(&self) -> GridIterator<T> {
        self.iterate_by(false)
    }

    pub fn grid_map<V: GridCell>(&self, f: impl Fn(GridPos, T) -> V) -> Grid<V> {
        let mut new_grid = Vec::new();
        for row_idx in 0..self.rows {
            let row = self.grid.get(row_idx).unwrap();
            let new_row: Vec<V> = row
                .iter()
                .enumerate()
                .map(|(col_idx, v)| f(GridPos::new(row_idx, col_idx), v.clone()))
                .collect();
            new_grid.push(new_row);
        }
        Grid {
            grid: new_grid,
            rows: self.rows,
            cols: self.cols,
        }
    }

    fn column_cell_ordering(left: &GridPos, right: &GridPos) -> Ordering {
        match left.col.cmp(&right.col) {
            Ordering::Equal => left.row.cmp(&right.row),
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    }
}
