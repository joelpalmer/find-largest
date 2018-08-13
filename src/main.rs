fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![2, 4, 37, 38, 42, 48];

    let result = largest(&number_list);
    println!("Largest number is {}", result);

    let char_list = vec!['w', 'd', 'p', 's', 'm', 'w', 'p'];

    let result = largest(&char_list);
    println!("Largest char is {}", result);
}
