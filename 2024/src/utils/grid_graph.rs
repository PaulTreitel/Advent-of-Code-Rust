use std::{cmp::Ordering, collections::{BinaryHeap, HashMap, HashSet, VecDeque}};

use petgraph::{graph::node_index, prelude::StableGraph, visit::NodeIndexable, Directed, EdgeType, Undirected};

use super::{graph_algos::GraphWrapper, grid::{Grid, GridCell, GridPos}};

impl<T: GridCell> Grid<T> {

    /* BFS/DFS Methods */

    pub fn bfs_first_match(
        &self,
        start: GridPos,
        has_edge: impl Fn((&GridPos, &T), (&GridPos, &T)) -> bool,
        matches: impl Fn(&GridPos, &T) -> bool,
    ) -> Option<(usize, GridPos)> {
        self.bfs(start, has_edge, matches, true, false)
            .first().copied()
    }

    pub fn bfs_all_matches(
        &self,
        start: GridPos,
        has_edge: impl Fn((&GridPos, &T), (&GridPos, &T)) -> bool,
        matches: impl Fn(&GridPos, &T) -> bool,
    ) -> Vec<(usize, GridPos)> {
        self.bfs(start, has_edge, matches, false, false)
    }

    pub fn dfs_first_match(
        &self,
        start: GridPos,
        has_edge: impl Fn((&GridPos, &T), (&GridPos, &T)) -> bool,
        matches: impl Fn(&GridPos, &T) -> bool,
    ) -> Option<(usize, GridPos)> {
        self.dfs(start, has_edge, matches, true, false)
            .first().copied()
    }

    pub fn dfs_all_matches(
        &self,
        start: GridPos,
        has_edge: impl Fn((&GridPos, &T), (&GridPos, &T)) -> bool,
        matches: impl Fn(&GridPos, &T) -> bool,
    ) -> Vec<(usize, GridPos)> {
        self.dfs(start, has_edge, matches, false, false)
    }

    pub fn bfs_dfs_full(
        &self,
        start: GridPos,
        has_edge: impl Fn((&GridPos, &T), (&GridPos, &T)) -> bool,
    ) -> HashMap<GridPos, (usize, &T)> {
        let mut all: Vec<(usize, GridPos)> = self
            .bfs(start, has_edge, |_, _| true, false, true)
            .iter()
            .map(|(i, p)| (*i, *p))
            .collect();
        all.sort_by(Grid::<T>::bfs_dfs_result_sort);
        all.dedup_by(|a, b| a.1 == b.1);
        all
            .iter()
            .map(|(i, pos)| (*pos, (*i, self.get(pos).unwrap())))
            .collect()
    }

    /* Helper Methods For BFS/DFS */

    fn bfs(
        &self,
        start: GridPos,
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
        start: GridPos,
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
        start: GridPos,
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
        qs_insert(&mut queue_stack, (0, start));
        if matches(&start, self.get(&start).unwrap()) {
            result.push((0, start));
        }
        while let Some((i, pos)) = qs_remove(&mut queue_stack) {
            visited.insert(pos);
            let curr = (&pos, self.get(&pos).unwrap());
            let new_positions = self.get_neighbors(pos);
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
                        qs_insert(&mut queue_stack, (i + 1, new_pos));
                    }
                } else if !visited.contains(&new_pos) {
                    qs_insert(&mut queue_stack, (i + 1, new_pos));
                }
            }
        }
        result
    }

    fn bfs_dfs_result_sort(a: &(usize, GridPos), b: &(usize, GridPos)) -> Ordering {
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

    fn get_neighbors(&self, pos: GridPos) -> Vec<GridPos> {
        match self.graph_edge_type() {
            super::direction::DirectionType::Orthogonal => pos.get_orthogonal_neighbors(),
            super::direction::DirectionType::Diagonal => pos.get_diag_neighbors(),
            super::direction::DirectionType::All => pos.get_all_neighbors(),
        }
    }

    /* Dijkstra's */

    pub fn dijkstra(
        &self,
        start: GridPos,
        edge_weight: impl Fn(&GridPos, &GridPos) -> Option<i64>
    ) -> HashMap<GridPos, (i64, Vec<GridPos>)> {
        let mut dists_paths = HashMap::with_capacity(self.rows() * self.cols());
        let mut heap = BinaryHeap::new();
        dists_paths.insert(start, (0, vec![]));
        heap.push((0i64, start));
        while let Some((cost, pos)) = heap.pop() {
            if dists_paths.contains_key(&pos) && cost > dists_paths.get(&pos).unwrap().0 {
                continue;
            }
            for neighbor in self.get_neighbors(pos) {
                let edge = edge_weight(&pos, &neighbor);
                if let Some(weight) = edge {
                    if !dists_paths.contains_key(&neighbor)
                        || cost + weight < dists_paths.get(&neighbor).unwrap().0
                    {
                        heap.push((cost + weight, neighbor));
                        dists_paths.entry(neighbor)
                            .and_modify(|x| {
                                x.0 = cost + weight;
                                x.1 = vec![pos];
                            })
                            .or_insert((cost + weight, vec![pos]));
                    } else if dists_paths.contains_key(&neighbor)
                        && cost + weight == dists_paths.get(&neighbor).unwrap().0
                    {
                        dists_paths.get_mut(&neighbor).unwrap().1.push(pos);
                    }
                }
            }
        }
        dists_paths
    }

    /* Converting into a petgraph */

    pub fn to_undir_graph<EWeight>(
        &self,
        is_node: impl Fn(GridPos, T) -> bool,
        has_edge: impl Fn((&GridPos, &T), (&GridPos, &T)) -> bool,
        edge_weight: impl Fn((&GridPos, &T), (&GridPos, &T)) -> EWeight,
    ) -> GraphWrapper<GridPos, EWeight, Undirected> {
        self.to_graph_internal::<EWeight, Undirected>(false, is_node, has_edge, edge_weight)
    }

    pub fn to_dir_graph<EWeight>(
        &self,
        is_node: impl Fn(GridPos, T) -> bool,
        has_edge: impl Fn((&GridPos, &T), (&GridPos, &T)) -> bool,
        edge_weight: impl Fn((&GridPos, &T), (&GridPos, &T)) -> EWeight,
    ) -> GraphWrapper<GridPos, EWeight, Directed> {
        self.to_graph_internal::<EWeight, Directed>(true, is_node, has_edge, edge_weight)
    }

    fn to_graph_internal<EWeight, EType: EdgeType>(
        &self,
        directed: bool,
        is_node: impl Fn(GridPos, T) -> bool,
        has_edge: impl Fn((&GridPos, &T), (&GridPos, &T)) -> bool,
        edge_weight: impl Fn((&GridPos, &T), (&GridPos, &T)) -> EWeight,
    ) -> GraphWrapper<GridPos, EWeight, EType> {
        let num_cells = self.rows() * self.cols();
        let mut pos_to_node_id = HashMap::with_capacity(num_cells);
        let mut graph = StableGraph::with_capacity(num_cells, 4 * num_cells);

        for (pos, val) in self.iter_by_rows() {
            if is_node(pos, val) {
                let id = graph.node_bound();
                pos_to_node_id.insert(pos, vec![node_index(id)]);
                graph.add_node(pos);
            }
        }

        for node_id in 0..graph.node_count() {
            let start_pos = graph[node_index(node_id)];
            let start = (&start_pos, self.get(&start_pos).unwrap());
            for neighbor_pos in start_pos.get_orthogonal_neighbors() {
                if !self.is_valid_cell(&neighbor_pos) {
                    continue;
                }
                let neighbor = (&neighbor_pos, self.get(&neighbor_pos).unwrap());
                if !has_edge(start, neighbor) {
                    continue;
                }

                let weight = edge_weight(start, neighbor);
                if let Some(neighbor_id) = pos_to_node_id.get(&neighbor_pos) {
                    let neighbor_id = *neighbor_id.first().unwrap();
                    if directed || !graph.contains_edge(neighbor_id, node_id.into()) {
                        graph.add_edge(node_id.into(), neighbor_id, weight);
                    }
                }
            }
        }
        GraphWrapper::new(graph, pos_to_node_id)
    }
}
