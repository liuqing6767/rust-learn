fn main() {
    let numbers = vec![1, 4, 9, 8];
    let max = largest(numbers);
    println!("numbers max is {}", max);

    let chars = vec!['a', 'c', 'd', 'b'];
    let max = largest(chars);
    println!("chars max is {}", max);
}



fn largest<T std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut tmp = &list[0]

    for item in list {
        if item > tmp {
            tmp = item;
        }
    }

    tmp
}