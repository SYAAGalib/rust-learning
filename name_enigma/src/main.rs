use std::io;

fn main() {
    let cases = take_input_make_int();

    let total_char = take_input_make_int();

    let check_string = check_enigma()
    let len_sring = check_string.len();

    println!("{}", len_sring);

}

fn take_input_make_int() -> u32{
    let mut test_case = String::new();
    io::stdin().read_line(&mut test_case)
        .expect("Failed");
    let test_case: u32 = test_case.trim().parse()
        .expect("Failed to read");
    test_case
}

fn check_enigma() -> String{
    let mut takes_string = String::new();
    io::stdin().read_line(&mut takes_string)
        .expect("Failed");
    let takes_string: String = takes_string.trim();

    
}