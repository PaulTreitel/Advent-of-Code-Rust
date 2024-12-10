use std::fmt::{Debug, Write};

use super::grid::{Grid, GridCell};

pub fn pretty_print_2d_vecs<T: Debug>(v: &Vec<Vec<T>>) {
    println!("[");
    for row in v {
        let p = row.iter().fold(String::new(), |mut s, t| {
            let _ = write!(s, "{:?}, ", t);
            s
        });
        println!("[{}]", &p[..p.len() - 2]);
    }
    println!("]");
}

pub fn pretty_print_grid<T: GridCell>(grid: &Grid<T>) {
    pretty_print_2d_vecs(&grid.grid_clone());
}
