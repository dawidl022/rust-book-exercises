fn main() {
    println!("{}", fibonacci(10));
}

fn fibonacci(n: i32) -> i128 {
    if n < 2 {
        return n.into();
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}
