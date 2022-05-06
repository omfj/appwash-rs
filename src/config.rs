// TODO: Add specified config path to use

pub struct User {
    pub email: String,
    pub password: String,
    pub token: String,
    // pub config: String,
}

impl User {
    pub fn new() -> Self {
        User {
            email: String::new(),
            password: String::new(),
            token: String::new(),
            // pub config: String,
        }
    }
}
