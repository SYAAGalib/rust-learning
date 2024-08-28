use std::io;

fn main() {
    let mut test_case = String::new();
    io::stdin().read_line(&mut test_case)
        .expect("Failed");
    let test_case: u32 = test_case.trim().parse()
        .expect("Failed to read");
    
    let mut index = 0;
    
    while index < test_case{
        let mut n = String::new();
        io::stdin().read_line(&mut n)
            .expect("Failed");
        let n: u32 = n.trim().parse()
            .expect("Failed to read");
        
        let total = {
            (n * (n + 1))/2
        };
        println!("{}", total);
        index = index + 1;
    }
}

