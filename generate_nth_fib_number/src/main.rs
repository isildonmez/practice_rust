use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = &args[1];
    let n = n.trim().parse().expect("Having a problem parsing the argument.");

    let fib_num = nth_fib_num(n);
    println!("{n}th fib number is {fib_num}")
}

// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233...
// 1, 2, 3, 4, 5, 6, 7,  8,  9, 10, 11, 12,  13,  14...
fn nth_fib_num(n: i32) -> i32 {
    if n == 1 {
        return 0
    } else if n == 2 {
        return 1
    } else if n > 2 {
        return nth_fib_num(n-1) + nth_fib_num(n-2)
    } else {
        // Assuming fib_num == 0 for the rest of the n
        return 0
    }
}
