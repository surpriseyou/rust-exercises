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

#[derive(Debug)]
pub struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Fibonacci {
    pub fn new(curr: u32, next: u32) -> Fibonacci {
        Fibonacci { curr, next }
    }

    pub fn array(count: usize) -> Vec<u32> {
        if count < 1 {
            ()
        }
        let fib = Fibonacci::new(0, 1);
        let take = fib.take(count - 1);
        let mut arr = vec![];
        arr.push(0);
        for x in take {
            arr.push(x)
        }
        arr
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}
