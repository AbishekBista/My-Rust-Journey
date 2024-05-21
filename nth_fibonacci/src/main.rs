use std::io;

fn main() {
    println!("Fibonacci Number Generator");

    let mut n = String::new();
    println!("Enter the nth term to be generated: ");
    io::stdin().read_line(&mut n).expect("Failed to read line");

    println!("Term number: {n}");

    let _term:u64 = fibonacci_number_generator(n.trim().parse().expect("Could not convert"));

    println!("The Nth term is {_term}");
}

fn fibonacci_number_generator(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    
    if n == 1 {
        a
    }
    else if n == 2 {
        b
    } else {
        for term in 3..=n {
            c = a + b;
            a = b;
            b = c;
        }
        c
    }

}