use std::fmt::Debug;


pub fn pretty_print_2d_vecs<T: Debug>(v: &Vec<Vec<T>>) {
    println!("[");
    for row in v {
        let p = row.iter().map(|t| format!("{:?}, ", t)).collect::<String>();
        println!("[{}]", &p[..p.len() - 2]);
    }
    println!("]");
}
