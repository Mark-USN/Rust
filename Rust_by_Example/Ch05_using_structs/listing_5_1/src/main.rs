#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("user1 = {:#?}", user1);

  //  user1.email = String::from("anotheremail@exlample.com");
    
  //  println!("user1 = {:#?}", user1);

    let user2 = build_user(String::from("anonymous@example.org"), String::from("anonymous")); 

    println!("user2 = {:#?}", user2);

    let user2 = User {
        email: String::from("anotheremail@exlample.com"),
        ..user1

    };

    println!("user2 = {:#?}", user2);
  //  println!("user1 = {:#?}", user1);

}

fn build_user(email: String, username: String) -> User {
    User{
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
