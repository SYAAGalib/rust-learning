enum Coin{
    Penny,
    Nickel,
    Dime,
    Quater,
}

fn value_in_cents(coin: Coin) -> u32 {
    let mut count = 1;
    match coin {
        
        Coin::Penny => {
            println!("Penny!");
            count = count +1;
            println!("wth!{}", count);
            count
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater => 25,
    }
}

fn main(){
    loop{
        value_in_cents(Coin::Penny);
        println!("wtf!{}", value_in_cents(Coin::Penny));

    }

}