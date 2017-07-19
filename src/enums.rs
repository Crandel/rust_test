pub fn enums_test() {
    let hulk = Hero::Strong(100);
    let flash = Hero::Fast;
    let superman = Hero::Info{name: "superman".to_owned(), secret: "Clark Kent".to_owned()};
    hulk.get_info();
    flash.get_info();
    superman.get_info();
}

enum Hero {
    Fast,
    Strong(i32),
    Info {name: String, secret: String}
}

impl Hero {
    fn get_info(self) {
        match self {
            Hero::Fast => println!("Fast"),
            Hero::Strong(i) => println!("Hero could lift {} tons", i),
            Hero:: Info {name, secret} => {
                println!("{} secret is {}", name, secret);
            }
        }
    }
}
