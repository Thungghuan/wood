pub struct Bot {
    qq: String,
}

impl Bot {
    pub fn new(qq: String) -> Self {
        Bot { qq }
    }

    pub fn start(&self) {
        println!("Bot qq is: {}", self.qq)
    }
}
