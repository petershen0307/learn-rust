fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];
    mut_reference(&number_list, &mut largest);
    println!("The largest number is {}", largest);
}

fn mut_reference(number_list: &Vec<i32>, largest: &mut i32) {
    for number in number_list {
        if number > largest {
            *largest = *number;
        }
    }
}
