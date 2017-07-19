pub fn test_structs() {

    let user = User {
        name: "Vitalii".to_owned(), surname: "Drevenchuk".to_owned(), age: 29, money: 100.00
    };

    println!("User {0} {1}", user.name, user.get_surname());
}

struct User {
    name: String,
    surname: String,
    age: i32,
    money: f64,
}

fn get_name(user: &User) -> &str {
    &user.name
}

impl User {
    pub fn get_surname(&self) -> &str {
        &self.surname
    }
}
