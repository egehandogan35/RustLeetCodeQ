use std::io;

fn main() {
    loop {
        let mut user_input = String::new();
        println!("Enter two integers separated by a space:");

        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let input_values: Vec<&str> = user_input.trim().split_whitespace().collect();
        let result: Result<Vec<i32>, _> = input_values
            .iter()
            .map(|&s| s.parse::<i32>())
            .collect();

        match result {
            Ok(parsed_values) => {
                let (x, y) = match parsed_values.as_slice() {
                    [x, y] => (*x, *y),
                    _ => {
                        println!("Error: Please enter exactly two integers separated by a space.");
                        continue; 
                    }
                };
                println!(" x = {}, y = {}", x, y);
                println!("Sum of values: {}", calculatesum(x, y));
                println!("Even or odd x and y are: {} and {}", even_odd(x), even_odd(y));
                break; 
            }
            Err(_) => {
                println!("Error: Please enter two valid integers separated by a space.");
            }
        }
    }

    
}

fn calculatesum(x: i32, y: i32) -> i32 {
    x + y
}

fn even_odd(x: i32) -> bool {
    x % 2 == 0
}
