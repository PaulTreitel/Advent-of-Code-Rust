// use graph_builder::prelude::*;

use std::collections::HashMap;

pub trait BfsDfs<Value, Position> {

    fn bfs_first_match(
        &self,
        start: &Position,
        has_edge: impl Fn((&Position, &Value), (&Position, &Value)) -> bool,
        matches: impl Fn(&Position, &Value) -> bool,
    ) -> Option<(usize, Position)>;

    fn bfs_all_matches(
        &self,
        start: &Position,
        has_edge: impl Fn((&Position, &Value), (&Position, &Value)) -> bool,
        matches: impl Fn(&Position, &Value) -> bool,
    ) -> Vec<(usize, Position)>;

    fn bfs_first_match_path(
        &self,
        start: &Position,
        has_edge: impl Fn((&Position, &Value), (&Position, &Value)) -> bool,
        matches: impl Fn(&Position, &Value) -> bool,
    ) -> Option<(usize, Position, Vec<Position>)>;

    fn dfs_first_match(
        &self,
        start: &Position,
        has_edge: impl Fn((&Position, &Value), (&Position, &Value)) -> bool,
        matches: impl Fn(&Position, &Value) -> bool,
    ) -> Option<(usize, Position)>;

    fn dfs_all_matches(
        &self,
        start: &Position,
        has_edge: impl Fn((&Position, &Value), (&Position, &Value)) -> bool,
        matches: impl Fn(&Position, &Value) -> bool,
    ) -> Vec<(usize, Position)>;

    fn dfs_first_match_path(
        &self,
        start: &Position,
        has_edge: impl Fn((&Position, &Value), (&Position, &Value)) -> bool,
        matches: impl Fn(&Position, &Value) -> bool,
    ) -> Option<(usize, Position, Vec<Position>)>;

    fn bfs_dfs_full(
        &self,
        start: &Position,
        has_edge: impl Fn((&Position, &Value), (&Position, &Value)) -> bool,
    ) -> HashMap<Position, (usize, &Value)>;
}
