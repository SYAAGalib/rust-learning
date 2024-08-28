use std::collections::HashMap;
use std::io;


fn main() {
    let mut map = HashMap::new();

    map.insert(String::from("Employee"), String::from("Department"));

    for _i in [0..100]{
        println!("enter name:");
        let mut employee_name = String::new();
        io::stdin().read_line(&mut employee_name).expect("failed!!!");

        println!("enter department:");
        let mut department = String::new();
        io::stdin().read_line(&mut department).expect("failed!!!");

        map.entry(employee_name).or_insert(department);
    }
    
    println!("{:#?}", map);

}
