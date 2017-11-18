pub fn enums_test() {
    let hulk = Hero::Strong(100);
    let flash = Hero::Fast;
    let superman = Hero::Info{
        name: "superman".to_owned(), 
        secret: "Clark Kent".to_owned()
    };
    hulk.get_info();
    flash.get_info();
    superman.get_info();

    let num = Some(6u8);
    let null = None;
    simple_add(num);
    simple_add(null);
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

fn simple_add(op: Option<u8>){
    match op {
        None => println!("None value"),
        Some(x) => println!("x value is {}", x),
    }
}
