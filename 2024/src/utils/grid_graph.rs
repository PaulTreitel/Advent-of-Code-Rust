use std::{cmp::Ordering, collections::{HashMap, HashSet, VecDeque}};

use super::{graph::BfsDfs, grid::{Grid, GridCell, GridPos}};

impl<T: GridCell> BfsDfs<T, GridPos> for Grid<T> {
    fn bfs_find_first_match(
        &self,
        start: &GridPos,
        has_edge: impl Fn((&GridPos, &T), (&GridPos, &T)) -> bool,
        matches: impl Fn(&GridPos, &T) -> bool,
    ) -> Option<(usize, GridPos)> {
        self.bfs(start, has_edge, matches, true, false)
            .first().copied()
    }

    fn bfs_find_all_matches(
        &self,
        start: &GridPos,
        has_edge: impl Fn((&GridPos, &T), (&GridPos, &T)) -> bool,
        matches: impl Fn(&GridPos, &T) -> bool,
    ) -> Vec<(usize, GridPos)> {
        self.bfs(start, has_edge, matches, false, false)
    }

    fn bfs_dfs_full(
        &self,
        start: &GridPos,
        has_edge: impl Fn((&GridPos, &T), (&GridPos, &T)) -> bool,
    ) -> HashMap<GridPos, (usize, &T)> {
        let mut all: Vec<(usize, GridPos)> = self
            .bfs(start, has_edge, |_, _| true, false, true)
            .iter()
            .map(|(i, p)| (*i, *p))
            .collect();
        all.sort_by(Grid::<T>::bfs_result_sort);
        all.dedup_by(|a, b| a.1 == b.1);
        all
            .iter()
            .map(|(i, pos)| (*pos, (*i, self.get(pos).unwrap())))
            .collect()
    }

    fn dfs_find_first_match(
        &self,
        start: &GridPos,
        has_edge: impl Fn((&GridPos, &T), (&GridPos, &T)) -> bool,
        matches: impl Fn(&GridPos, &T) -> bool,
    ) -> Option<(usize, GridPos)> {
        self.dfs(start, has_edge, matches, true, false).first().copied()
    }

    fn dfs_find_all_matches(
        &self,
        start: &GridPos,
        has_edge: impl Fn((&GridPos, &T), (&GridPos, &T)) -> bool,
        matches: impl Fn(&GridPos, &T) -> bool,
    ) -> Vec<(usize, GridPos)> {
        self.dfs(start, has_edge, matches, false, false)
    }
}

/* Necessary Helper Functions For BFS/DFS */
impl<T: GridCell> Grid<T> {
    fn bfs(
        &self,
        start: &GridPos,
        has_edge: impl Fn((&GridPos, &T), (&GridPos, &T)) -> bool,
        matches: impl Fn(&GridPos, &T) -> bool,
        only_find_first: bool,
        continue_on_match: bool,
    ) -> Vec<(usize, GridPos)> {
        self.bfs_dfs_internal(
            start,
            has_edge,
            matches,
            only_find_first,
            continue_on_match,
            VecDeque::new(),
            |s, i| s.push_back(i),
            |s| s.pop_front())
    }

    fn dfs(
        &self,
        start: &GridPos,
        has_edge: impl Fn((&GridPos, &T), (&GridPos, &T)) -> bool,
        matches: impl Fn(&GridPos, &T) -> bool,
        only_find_first: bool,
        continue_on_match: bool,
    ) -> Vec<(usize, GridPos)> {
        self.bfs_dfs_internal(
            start,
            has_edge,
            matches,
            only_find_first,
            continue_on_match,
            Vec::new(),
            |s, i| s.push(i),
            |s| s.pop())
    }

    fn bfs_dfs_internal<Storage: Clone>(
        &self,
        start: &GridPos,
        has_edge: impl Fn((&GridPos, &T), (&GridPos, &T)) -> bool,
        matches: impl Fn(&GridPos, &T) -> bool,
        only_find_first: bool,
        continue_on_match: bool,
        queue_stack: Storage,
        qs_insert: impl Fn(&mut Storage, (usize, GridPos)),
        qs_remove: impl Fn(&mut Storage) -> Option<(usize, GridPos)>,
    ) -> Vec<(usize, GridPos)> {
        let mut result = vec![];
        let mut visited = HashSet::new();
        let mut queue_stack = queue_stack.clone();
        qs_insert(&mut queue_stack, (0, *start));
        if matches(start, self.get(start).unwrap()) {
            result.push((0, *start));
        }
        while let Some((i, pos)) = qs_remove(&mut queue_stack) {
            visited.insert(pos);
            let curr = (&pos, self.get(&pos).unwrap());
            let new_positions = match self.graph_edge_type() {
                super::direction::DirectionType::Orthogonal => pos.get_orthogonal_neighbors(),
                super::direction::DirectionType::Diagonal => pos.get_diag_neighbors(),
                super::direction::DirectionType::All => pos.get_all_neighbors(),
            };
            for new_pos in new_positions {
                if !self.is_valid_cell(&new_pos) {
                    continue;
                }
                let new = (&new_pos, self.get(&new_pos).unwrap());
                if !has_edge(curr, new) {
                    continue;
                }
                if matches(&new_pos, new.1) {
                    if only_find_first {
                        return vec![(i + 1, *new.0)];
                    } else {
                        result.push((i + 1, *new.0));
                    }
                    if continue_on_match && !visited.contains(&new_pos) {
                        qs_insert(&mut queue_stack, (i + 1, new_pos))
                    }
                } else if !visited.contains(&new_pos) {
                    qs_insert(&mut queue_stack, (i + 1, new_pos))
                }
            }
        }
        result
    }

    fn bfs_result_sort(a: &(usize, GridPos), b: &(usize, GridPos)) -> Ordering {
        let row_cmp = a.1.row.cmp(&b.1.row);
        let col_cmp = a.1.col.cmp(&b.1.col);
        let iter_cmp = a.0.cmp(&b.0);
        if row_cmp != Ordering::Equal {
            row_cmp
        } else if col_cmp != Ordering::Equal {
            col_cmp
        } else {
            iter_cmp
        }
    }

    pub fn from_bfs_dfs(search_results: HashMap<GridPos, (usize, &T)>) -> Grid<(usize, &T)> {
        let mut split_by_row = vec![];
        for (pos, val) in search_results {
            while split_by_row.len() <= pos.row {
                split_by_row.push(vec![]);
            }
            split_by_row.get_mut(pos.row).unwrap().push((val.0, pos, val.1));
        }
        for row in 0..split_by_row.len() {
            split_by_row
                .get_mut(row)
                .unwrap()
                .sort_by(|(_, p1, _), (_, p2, _)| p1.cmp(p2));
        }
        let new_grid: Vec<Vec<(usize, &T)>> = split_by_row.iter()
            .map(|row| row.iter()
                .map(|(i, _, val)| (*i, *val))
                .collect()
            )
            .collect();
        Grid::from(new_grid)
    }
}
