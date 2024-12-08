use std::{
    cmp::{min, Ordering},
    fmt::{Debug, Display},
};

use super::show;
pub trait GridCell: Clone + PartialEq + Ord + Debug {}
impl<T> GridCell for T where T: Clone + PartialEq + Ord + Debug {}

#[derive(Debug, Clone)]
pub struct GridIterator<GridCell> {
    grid_idxs: Vec<((usize, usize), GridCell)>,
}

impl<GridCell> Iterator for GridIterator<GridCell> {
    type Item = ((usize, usize), GridCell);

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

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.grid.get(row)?.get(col)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.grid.get_mut(row)?.get_mut(col)
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

    pub fn index_of(&self, eq: &T) -> Option<(usize, usize)> {
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.grid.get(r)?.get(c)?.eq(eq) {
                    return Some((r, c));
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

    pub fn is_valid_cell(&self, row: usize, col: usize) -> bool {
        row < self.rows && col < self.cols
    }

    pub fn valid_directional_scan(
        &self,
        row: usize,
        col: usize,
        offset: (i32, i32),
        scan_len: usize,
    ) -> bool {
        let row_min_fails = offset.0 < 0 && row < scan_len;
        let row_max_fails = offset.0 > 0 && row >= self.rows - scan_len;
        let col_min_fails = offset.1 < 0 && col < scan_len;
        let col_max_fails = offset.1 > 0 && col >= self.cols - scan_len;
        !(row_max_fails || row_min_fails || col_max_fails || col_min_fails)
    }

    pub fn scan_direction(
        &self,
        row: usize,
        col: usize,
        offset: (i32, i32),
        scan_len: usize,
    ) -> Option<Vec<((usize, usize), T)>> {
        if offset.0 == 0 && offset.1 == 0 {
            return None;
        }
        if !self.valid_directional_scan(row, col, offset, scan_len - 1) {
            return None;
        }
        let mut scan_result = Vec::new();
        for i in 0..scan_len as i32 {
            let target_row = (row as i32 + offset.0 * i) as usize;
            let target_col = (col as i32 + offset.1 * i) as usize;
            let val = self.get(target_row, target_col)?.clone();
            scan_result.push(((target_row, target_col), val));
        }
        Some(scan_result)
    }

    fn max_scan_iterations(&self, row: usize, col: usize, offset: (i32, i32)) -> i32 {
        let until_rows_end = match offset.0.cmp(&0) {
            Ordering::Less => row as i32 / offset.0.abs(),
            Ordering::Equal => self.cols as i32,
            Ordering::Greater => (self.rows as i32 - row as i32) / offset.0,
        };
        let until_cols_end = match offset.1.cmp(&0) {
            Ordering::Less => col as i32 / offset.1.abs(),
            Ordering::Equal => self.rows as i32,
            Ordering::Greater => (self.cols as i32 - col as i32) / offset.1,
        };
        min(until_rows_end, until_cols_end) + 1
    }

    pub fn scan_direction_until(
        &self,
        row: usize,
        col: usize,
        offset: (i32, i32),
        stop_condition: impl Fn((usize, usize), &T) -> bool,
    ) -> Option<Vec<((usize, usize), T)>> {
        if offset.0 == 0 && offset.1 == 0 {
            return None;
        }
        let max_scan_len = self.max_scan_iterations(row, col, offset);
        if max_scan_len <= 0 {
            return None;
        }
        let mut scan_result = Vec::new();
        for i in 0..max_scan_len {
            let target_row = (row as i32 + offset.0 * i) as usize;
            let target_col = (col as i32 + offset.1 * i) as usize;
            match self.get(target_row, target_col) {
                Some(c) => {
                    if stop_condition((target_row, target_col), c) {
                        break;
                    }
                    scan_result.push(((target_row, target_col), c.clone()))
                },
                None => return Some(scan_result)
            }

        }
        Some(scan_result)
    }

    fn get_iterator_grid(&self) -> Vec<((usize, usize), T)> {
        let mut new_grid = Vec::new();
        for row_idx in 0..self.rows {
            let row = self.grid.get(row_idx).unwrap();
            let new_row: Vec<((usize, usize), T)> = row
                .iter()
                .enumerate()
                .map(|(col_idx, v)| ((row_idx, col_idx), v.clone()))
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

    pub fn iterate_by_rows(&self) -> GridIterator<T> {
        self.iterate_by(true)
    }

    pub fn iterate_by_cols(&self) -> GridIterator<T> {
        self.iterate_by(false)
    }

    pub fn grid_map<V: GridCell>(&self, f: impl Fn((usize, usize), T) -> V) -> Grid<V> {
        let mut new_grid = Vec::new();
        for row_idx in 0..self.rows {
            let row = self.grid.get(row_idx).unwrap();
            let new_row: Vec<V> = row
                .iter()
                .enumerate()
                .map(|(col_idx, v)| f((row_idx, col_idx), v.clone()))
                .collect();
            new_grid.push(new_row);
        }
        Grid {
            grid: new_grid,
            rows: self.rows,
            cols: self.cols,
        }
    }

    fn column_cell_ordering(left: &(usize, usize), right: &(usize, usize)) -> Ordering {
        match left.1.cmp(&right.1) {
            Ordering::Equal => left.0.cmp(&right.0),
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    }

    pub fn print(&self) {
        let mut cells: Vec<Vec<&T>> = Vec::new();
        for row_idx in 0..self.rows {
            let row: Vec<&T> = (0..self.cols)
                .map(|col_idx| self.get(row_idx, col_idx).unwrap())
                .collect();
            cells.push(row);
        }
        show::pretty_print_2d_vecs(&cells);
    }
}
