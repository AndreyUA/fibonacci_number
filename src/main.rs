fn main() {
    let count = 160;
    let answer = fibo(count);

    println!("The {} number ---> {}", count, answer);
}

fn fibo(num: u32) -> u128 {
    let mut prev_result: u128 = 1;

    let mut result: u128 = 0;

    for index in 0..num {
        if index == 0 {
            result = 0;
        } else if index == 1 {
            result = 1;
        } else {
            let prev_prev_result = prev_result;
            prev_result = result;
            result = prev_prev_result + prev_result;
        };
    }

    result
}
