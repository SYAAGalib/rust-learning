use std::io;

fn get_input() -> u64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().parse().expect("Please type a number!");
    input
}

fn is_valid(card_number: &u64) {
    let mut sum = 0;

    for (i, digit) in card_number.to_string().chars().rev().enumerate() {
        let digit = digit.to_digit(10).unwrap();
        if i % 2 == 1 {
            sum += digit;
        } else {
            sum += digit * 2;
        }
    }

    println!("Sum: {}", sum);

    if sum % 10 == 0 {
        println!("This is a valid credit card number");
    } else {
        println!("This is not a valid credit card number");
    }
}

fn check_card_type(card_number: &u64) {
    let card_number = card_number.to_string();
    let first_two_digits = &card_number[0..2];
    let first_digit = &card_number[0..1];

    if card_number.len() == 15 && (first_two_digits == "34" || first_two_digits == "37") {
        println!("This is an American Express card");
    } else if card_number.len() == 16 && (first_two_digits >= "51" && first_two_digits <= "55") {
        println!("This is a Mastercard");
    } else if (card_number.len() == 13 || card_number.len() == 16) && first_digit == "4" {
        println!("This is a Visa card");
    } else {
        println!("This is an unknown card");
    }

}
fn main() {
    println!("Enter your credit card number: ");
    let card_number = get_input();
    is_valid(&card_number);
    check_card_type(&card_number);
}

