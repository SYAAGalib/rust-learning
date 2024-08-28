fn main() {
    let a_list = [2, 4, 6, 8];
    let a_list_lem = a_list.len();
    println!("len of list{}", a_list_lem);

    for element in a_list.iter() {
        println!("{}", element);
    }

    for element in a_list.iter().rev() {
        println!("{}", element);
    }

    for number in (1..4).rev(){
        println!("{}", number);
    }
}
