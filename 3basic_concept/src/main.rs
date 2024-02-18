use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n1: u32 = n.trim().parse().expect("bad string");
    let n1 = fbn(n1);
    println!("{n} {n1} {n1}");
}


fn fbn(n: u32) -> i32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    return fbn(n-2) + fbn(n-1);
}
