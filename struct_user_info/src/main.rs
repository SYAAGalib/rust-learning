#[derive(Debug)]
struct User{
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}

fn main() {
    let user1 = User{
        email: String::from("ahsanullahalgalib@outlook.com"),
        username: String::from("syaagalib"),
        active: true,
        sign_in_count: 1,

    };

    let user2 = User{
        email: String::from("ahsanullahalgalib@gmail.com"),
        username: String::from("GAlib"),
        ..user1
    };

    println!("{:#?}", user1);
}
