use std::io;

fn main() {
    println!("Enter a index number: ");

    //takes input
    let n = take_string_return_int();

    //calculates n-th fibonacci number
    let fib_n_th = fib_n_th(&n);

    //prints the n-th number
    println!("the n'th fibonacci number is {}.", fib_n_th);
}


//TAKES INPUT AS STRING AND TURNS THEM INTO INTEGERS
fn take_string_return_int() -> u32{
    let mut n = String::new();
    io::stdin().read_line(&mut n)
        .expect("Failed to read line");
    let n: u32 = n.trim().parse()
        .expect("Something");

    n
}


//CALCULATES N-TH FIBONACCI NUMBER
fn fib_n_th(&n: &u32) -> u32{
    let mut index = 0;
    let mut fib_fisrt = 0;
    let mut fib_second = 1;
    let mut fib_n_th = 0;
    
    while index < n-2 {
        fib_n_th = fib_second + fib_fisrt;
        fib_fisrt = fib_second;
        fib_second = fib_n_th;
        index = index + 1;
       
    }

    fib_n_th
}