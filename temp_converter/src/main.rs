use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = &args[1];
    let f: i32 = f.trim().parse().expect("I can't parse the argument.");

    let c = f_to_c(f);
    println!("{f} Fahrenheit equals {c} Celsius")
}

fn f_to_c(f: i32) -> i32 {
    return (f - 32) * 5 / 9
}
