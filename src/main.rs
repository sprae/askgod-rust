use std::io;

fn read_question() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input
        }
        Err(error) => {
            println!("error: {}", error);
            input
        }
    }
}

fn string_to_vector(input: String) -> Vec<u8> {
    input.into_bytes()
}



fn sum_vector(input: Vec<u8>) -> u32 {
    input.iter().fold(0, |acc, &x| acc+x as u32)
}

fn print_answer(value: u32) {
    if value % 2 == 1 {
        println!("Tak!");
    } else {
        println!("Nie!");
    }
}

fn main() {
    let question = read_question();
    let text_binary_vector = string_to_vector(question);
    let text_sum = sum_vector(text_binary_vector);
    print_answer(text_sum);
}