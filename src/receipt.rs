use crate::{
    company::Company,
    helpers::{auto_pad::auto_pad, left_pad::left_pad, round::round},
    product::Product,
};
#[allow(dead_code)]
pub struct Receipt {
    products: Vec<Product>,
    firma: Company,
}
#[allow(dead_code)]
impl Receipt {
    pub fn print(&mut self) {
        println!("╔═════════════════════════╗");
        println!("║{}║", auto_pad(&self.firma.name, 25));
        println!("║{}║", auto_pad(&self.firma.street, 25));
        println!("║{}║", auto_pad(&self.firma.city, 25));
        println!("║{}║", auto_pad(&self.firma.phone, 25));
        println!("╚═════════════════════════╝");
        let mut sum: f32 = 0 as f32;
        for product in &self.products {
            println!("{}", product.name);
            println!(
                "{}{}",
                auto_pad(&format!("{}x", product.amount), 5),
                left_pad(&format!("{}", &round(product.single_price,2)), 21)
            );
            sum += product.amount as f32 * product.single_price;
        }
        println!("══════════════════════════");
        println!(
            "Summe CHF{}",
            left_pad(&round(sum, 2).to_string(), 17)
        );
        println!("══════════════════════════");
    }
    pub fn new(firma: Company) -> Receipt {
        Receipt {
            products: vec![],
            firma: firma,
        }
    }
    pub fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }
}
