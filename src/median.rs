use std::io;

pub fn get_median() {
    'set_number_list_loop: loop {
        println!("Set a list of numbers separated by a comma.");

        let mut number_list = String::new();

        io::stdin()
            .read_line(&mut number_list)
            .expect("Failed to read line");

        let number_list = number_list.trim().split(",");

        let mut number_vec: Vec<i32> = Vec::new();

        for number in number_list {
            let parsed_number: i32 = match number.trim().parse() {
                Ok(num) => num,
                Err(_) => continue 'set_number_list_loop,
            };

            number_vec.push(parsed_number);
        }

        number_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mid = number_vec.len() / 2;
        return println!("The median is: {}", number_vec[mid]);
    }
}
