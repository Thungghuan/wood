pub struct Http {
    qq: String,
}

impl Http {
    pub fn new(qq: &str) -> Self {
        Http {
            qq: String::from(qq),
        }
    }
}
