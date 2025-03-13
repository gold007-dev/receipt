#[allow(dead_code)]
pub struct Product {
    pub name: String,
    pub amount: i32,
    pub single_price: f32,
}
impl Product {
    pub fn new(name: &str, amount: i32, single_price: f32) -> Product {
        Product {
            name: name.to_string(),
            amount,
            single_price,
        }
    }
}
