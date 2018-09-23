extern crate base64;

pub struct Basic {
    username: String,
    password: String,
}

impl Basic {
    pub fn new(username: String, password: String) -> Basic {
        Basic {
            username: username,
            password: password,
        }
    }
    pub fn encode_tostr(&self) -> String {
        let user_data = format!("{}:{}", self.username, self.password);
        let b64 = base64::encode(user_data.as_bytes());
        println!("{}", b64);
        b64
    }
}
