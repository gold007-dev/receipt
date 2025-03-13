#[allow(dead_code)]
pub struct Company {
    pub name: String,
    pub street: String,
    pub city: String,
    pub phone: String,
}

impl Company {
    pub fn new(name: &str, street: &str, city: &str, phone: &str) -> Company {
        Company { name: name.to_string(), street: street.to_string(), city: city.to_string(), phone: phone.to_string() }
    }
}