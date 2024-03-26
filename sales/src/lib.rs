
#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Self {
            items: vec![],
            receipt: vec![]
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        self.items.push(s.products.iter()
            .find(|(p, _)| *p == ele)
            .expect("Could not find product")
            .clone()
        );
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let free_count = self.items.len() / 3;
        let mut ordered = self.items.clone();
        ordered.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
        let total_discount = ordered.iter()
            .take(free_count)
            .fold(0.0, |acc, (_, price)| acc + price);
        let total_sum = self.items.iter()
            .fold(0.0, |acc, (_, price)| acc + price);
        let discount = 1.0 - total_discount / total_sum;
        self.receipt = ordered.into_iter()
            .map(|(_, price)| price)
            .map(|p| ((p * discount) * 100.0).round() / 100.0)
            .collect();
        self.receipt.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let store = Store::new(vec![
            (String::from("product A"), 1.23),
            (String::from("product B"), 23.1),
            (String::from("product C"), 3.12)]);
        println!("{:?}", store);
        let mut cart = Cart::new();
        cart.insert_item(&store, String::from("product A"));
        cart.insert_item(&store, String::from("product B"));
        cart.insert_item(&store, String::from("product C"));
        println!("{:?}", cart.generate_receipt());
        println!("{:?}", cart);
    }
}