use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s)
        .expect("failed to read line");

    let s_copy = s.clone();
    let x: u32 = s_copy.len().try_into().unwrap();

   // println!("{}",x);
    
    let vowel = "aeiouAEIOU";
    for i in s.chars(){
        for v in vowel.chars(){
            if i != v{
                println!("{}-{}ay", &s[1..x], &s[0..1]);
                break
            } else {
                println!("{}-hay", &s[0..x]);
                break
            }
        }
        break
    }

 //   let first = &s[0];

}
