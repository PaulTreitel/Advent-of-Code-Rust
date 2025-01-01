use std::{collections::{HashMap, HashSet}, fmt::{Debug, Write}};

use super::grid::{Grid, GridCell};

pub fn pretty_print_2d_vecs<T: Debug>(v: &Vec<Vec<T>>, long_values: bool) {
    println!("[");
    for row in v {
        let p = row.iter().fold(String::new(), |mut s, t| {
            let _ = write!(s, "{:?}, ", t);
            s
        });
        if long_values {
            println!("\t[{}]", &p[..p.len() - 2]);
        } else {
            println!("[{}]", &p[..p.len() - 2]);
        }
    }
    println!("]");
}

pub fn pretty_print_grid<T: GridCell>(grid: &Grid<T>, long_values: bool) {
    pretty_print_2d_vecs(&grid.grid_clone(), long_values);
}

pub fn pretty_print_hmap<K: Debug, V: Debug>(h: &HashMap<K, V>, long_values: bool, newline: bool) {
    pretty_print_hmap_internal(h, long_values, "", newline);
}

pub fn pretty_print_hmap_of_hmaps<K: Debug, KK: Debug, V: Debug>(
    h: &HashMap<K, HashMap<KK, V>>,
    long_values: bool,
    newline: bool
) {
    for (k, hmap) in h {
        println!("{:?} ->", k);
        pretty_print_hmap_internal(hmap, long_values, "\t", false);
        if newline {
            println!();
        }
    }
}

fn pretty_print_hmap_internal<K: Debug, V: Debug>(
    h: &HashMap<K, V>,
    long_values: bool,
    prefix: &str,
    newline: bool
) {
    for (k, v) in h {
        if long_values {
            println!("{}{:?} ->\n\t{:?}", prefix, k, v);
        } else {
            println!("{}{:?} -> {:?}", prefix, k, v);
        }
    }
    if newline {
        println!();
    }
}

pub fn pretty_print_hset<T: Debug>(s: &HashSet<T>) {
    println!("{{");
    for v in s {
        println!("\t{:?}", v);
    }
    println!("}}");
}
