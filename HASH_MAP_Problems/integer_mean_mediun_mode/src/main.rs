// PROBLEM !!!!
//   Given a list of integers, use a vector and return the mean (the average value), 
//   median (when sorted, the value in the middle position), 
//   and mode (the value that occurs most often; a hash map will be helpful here) of the list.
               
use std::collections::HashMap;

fn main() {

    let integers: Vec<i32> = vec![5,6,4,7,6,4,1,7,9,4,45,7];
    let mut count = 0;
    for i in &integers{
        count += i; 
    }


    let len = &integers.len();
    println!("{}", count/2);            // how to make count float here. and how to control digits after (.) point.

    if len % 2 == 0{
        let middle_1 = len/2;
        let middle_2 = &middle_1 + 1;

        println!("The middles are {} {}", middle_1, middle_2);
    } else {
        let middle = (len + 1) / 2;
        println!("The middles is {}", middle);
    }

    let mut map = HashMap::new();

    for i in integers{
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
