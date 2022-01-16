use std::io;

fn main() {
    println!("Welcome to Fibonacci numbers calculator");

    loop {
        println!("Choose the number between 0 and 180: ");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        if number < 1 {
            println!("Please enter a number more then 0!");
            continue;
        }

        if number > 180 {
            println!("Please enter a number less then 180!");
            continue;
        }

        let answer = fibo(number);

        println!("The {} number ---> {}", number, answer);

        break;
    }
}

fn fibo(num: u32) -> u128 {
    let mut prev_result: u128 = 1;

    let mut result: u128 = 0;

    for index in 0..num {
        if index == 0 {
            result = 0;
        } else if index == 1 {
            result = 1;
        } else if index == 2 {
            result = 1;
        } else {
            let prev_prev_result = prev_result;
            prev_result = result;
            result = prev_prev_result + prev_result;
        };
    }

    result
}
