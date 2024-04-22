fn mut_main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];
    mut_reference(&number_list, &mut largest);
    println!("The largest number is {}", largest);
    let a = std::thread::spawn(move || {
        let mut input_buffer = String::new();
        std::io::stdin().read_line(&mut input_buffer).unwrap();
        println!("input_buffer={}", input_buffer);
    });
    a.join().unwrap();
}

fn mut_reference(number_list: &Vec<i32>, largest: &mut i32) {
    for number in number_list {
        if number > largest {
            *largest = *number;
        }
    }
}
