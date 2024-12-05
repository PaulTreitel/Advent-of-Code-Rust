use std::{cmp::Ordering, collections::HashMap};

pub const CARDINAL_DIRECTION_OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, -1)];
pub const DIAG_DIRECTION_OFFSETS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];
pub const ALL_DIRECTION_OFFSETS: [(i32, i32); 8] = [
    (-1, 0),
    (0, -1),
    (1, 0),
    (0, -1),
    (-1, -1),
    (-1, 1),
    (1, 1),
    (1, -1),
];

/* When scanning for sequences in a grid, one may want to scan just right/down
 * orthogonally and just down-left/down-right diagonally to avoid getting
 * duplicates from scanning the same sequence from above and below
 */
pub const DOWN_RIGHT_CARDINAL_OFFSETS: [(i32, i32); 2] = [(1, 0), (0, 1)];
pub const DOWN_DIAG_OFFSETS: [(i32, i32); 2] = [(1, 1), (1, -1)];
pub const ALL_DOWN_OFFSETS: [(i32, i32); 4] = [(1, 0), (0, 1), (1, 1), (1, -1)];

#[derive(Debug, Clone)]
pub struct Grid<T: Clone + PartialEq> {
    grid: HashMap<(usize, usize), T>,
    rows: usize,
    cols: usize,
}

#[derive(Debug, Clone)]
pub struct GridIterator {
    grid_idxs: Vec<(usize, usize)>,
}

impl Iterator for GridIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.grid_idxs.pop()
    }
}

impl<T: Clone + PartialEq> Grid<T> {
    pub fn from(grid: Vec<Vec<T>>) -> Self {
        let mut new_grid = HashMap::new();
        let rows = grid.len();
        if rows == 0 {
            panic!("Grid is empty!");
        }
        let cols = grid.first().unwrap().len();
        for r in 0..rows {
            for c in 0..cols {
                let v = grid.get(r).unwrap().get(c).unwrap();
                new_grid.insert((r, c), v.clone());
            }
        }
        Grid {
            grid: new_grid,
            rows,
            cols,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn valid_cell(&self, row: usize, col: usize) -> bool {
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

    pub fn get_directional_scan(
        &self,
        row: usize,
        col: usize,
        offset: (i32, i32),
        scan_len: usize,
    ) -> Option<Vec<T>> {
        let mut scan_result = Vec::new();
        if !self.valid_directional_scan(row, col, offset, scan_len - 1) {
            return None;
        }
        for i in 0..scan_len as i32 {
            let target_cell = (
                (row as i32 + offset.0 * i) as usize,
                (col as i32 + offset.1 * i) as usize,
            );
            scan_result.push(self.grid.get(&target_cell)?.clone());
        }
        Some(scan_result)
    }

    pub fn cell_eq(&self, row: usize, col: usize, other: &T) -> bool {
        if !self.valid_cell(row, col) {
            return false;
        }
        if let Some(cell) = self.grid.get(&(row, col)) {
            cell.eq(other)
        } else {
            false
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.grid.get(&(row, col))
    }

    pub fn iterate_by_rows(&self) -> GridIterator {
        let mut cell_positions: Vec<(usize, usize)> = self.grid.keys().copied().collect();
        cell_positions.sort();
        cell_positions.reverse();
        let x = GridIterator {
            grid_idxs: cell_positions,
        };
        x.into_iter()
    }

    pub fn iterate_by_cols(&self) -> GridIterator {
        let mut cell_positions: Vec<(usize, usize)> = self.grid.keys().copied().collect();
        cell_positions.sort_by(|x, y| {
            let col_order = x.1.cmp(&y.1);
            if let Ordering::Equal = col_order {
                x.0.cmp(&y.0)
            } else {
                col_order
            }
        });
        cell_positions.reverse();
        let x = GridIterator {
            grid_idxs: cell_positions,
        };
        x.into_iter()
    }
}
