use company::Company;
use product::Product;
use receipt::Receipt;

mod company;
mod product;
mod receipt;
mod helpers;

fn main() {
    let firma: Company = Company::new("FreitagsrundenShop 042","Marchstr. 23", "10587 Berlin", "☏ 030 314 213 86");
    let mut receipt: Receipt = Receipt::new(firma);
    receipt.add_product(Product::new("15 Fischstäbchen",3,1.79));
    receipt.add_product(Product::new("Steaks",5,3.99));
    receipt.add_product(Product::new("Naturelle (1l)",6,4.99));
    receipt.add_product(Product::new("Magnum Eis Creme",2,2.99));
    receipt.print();
}
