use std::io;

mod company;
mod median;
mod pig_latin;

fn main() {
    loop {
        println!("Choose one:");
        println!("1. Get median to a set of numbers.");
        println!("2. Translate text to pig Latin.");
        println!("3. Organize employees of a company.");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice = choice.trim();

        match choice {
            "1" => {
                median::get_median();
                break;
            }
            "2" => {
                pig_latin::get_translation();
                break;
            }
            "3" => {
                company::set_employees();
                break;
            }
            _ => continue,
        }
    }
}
