pub fn fibonacci(val: i32) -> i32 {
    let mut val = val;
    if val <= 1 {
        val
    } else {
        val = fibonacci(val - 1) + fibonacci(val - 2);
        val
    }
}

pub fn multiplication_table() {
    for i in 1..10 {
        for j in 1..i + 1 {
            print!("{} * {} = {}  ", j, i, i * j);
        }
        println!();
    }
}