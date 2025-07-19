struct Product {
    name: String,
    price: f32,
    stock: u32
}

impl Product {
    fn in_stock(&self) -> bool {
        self.stock > 0
    }
}

fn main(){
    let product1 = Product {
        name: String::from("Laptop"),
        price: 999.99,
        stock: 10,
    };

    let product2 = Product {
        name: String::from("Smartphone"),
        price: 499.99,
        stock: 0,
    };

    println!("Product 1: {}, Price: ${}, In Stock: {}", product1.name, product1.price, product1.in_stock());
    println!("Product 2: {}, Price: ${}, In Stock: {}", product2.name, product2.price, product2.in_stock());
}