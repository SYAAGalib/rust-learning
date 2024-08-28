use std::io;

fn main() {
    println!("Which temperature do you want to covert?\n 1. Celsius to Fahrenheit (press 1) \n 2. Fahrenheit to Celsius (press 2)");

    let mut convert = String::new();
    io::stdin().read_line(&mut convert)
        .expect("failed to readline!");
    let convert: i32 = convert.trim().parse()
        .expect("failed to readline!");

    println!("Enter the temperature in number:");

    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature)
        .expect("failed to readline!");
    let temperature: i32 = temperature.trim().parse()
        .expect("failed to readline!");


    if convert == 1{
        // FOr C to F
        let _temperature = (9*temperature + 160) / 5;
        println!("Now the temperature is {}F", _temperature);
    } else {
        // For F to C
        let _temperature = (5*temperature -160) / 9;
        println!("Now the temperature is {}C", _temperature);
    }

    


}
