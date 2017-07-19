pub fn test_structs() {

    let mut user = User {
        name: "Vitalii".to_owned(), surname: "Drevenchuk".to_owned(), age: 29, money: 100.00
    };

    println!("User {0} {1}", user.name, user.get_surname());
    let mut cow = Cow {
        sound: "Myyy".to_owned(), color: "Black".to_owned(), age: 2
    };

    println!("{}", user.say());
    println!("{}", cow.say());
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

struct Cow {
    sound: String,
    color: String,
    age: i32,
}

trait SoundMaker {
    fn say(&self) -> String;
}

impl User {
    fn say(&self) -> String {
        format!("User {} say hello!", &self.name)
    }
}

impl Cow {
    fn say(&self) -> String {
        format!("{} cow say {}", &self.color, &self.sound)
    }
}
