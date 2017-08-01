pub fn structs_test() {

    let user = User {
        name: "Vitalii".to_owned(), surname: "Drevenchuk".to_owned(), age: 29, money: 100.00
    };

    println!("User {0} {1}", user.name, user.get_surname());
    let cow = Cow {
        sound: "Myyy".to_owned(), color: "Black".to_owned(), age: 2
    };

    println!("{}", say_something(&user));
    println!("{}", say_something(&cow));
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

fn say_something(sm: &SoundMaker) -> String {
    sm.say()
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

impl SoundMaker for User {
    fn say(&self) -> String {
        format!("User {} say hello!", &self.name)
    }
}

impl SoundMaker for Cow {
    fn say(&self) -> String {
        format!("{} cow say {}", &self.color, &self.sound)
    }
}
