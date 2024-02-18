use std::io;

fn main() {
    let a = [1, 2, 3];

    println!("Please enter index");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered want a number");

    let ele = a[index];
    println!("{index}:{ele}");
}