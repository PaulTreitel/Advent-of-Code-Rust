use std::fmt::{Debug, Write};

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
