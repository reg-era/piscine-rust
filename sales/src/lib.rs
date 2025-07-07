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
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        let (_, price) = s.products.iter().find(|elem| elem.0 == ele).unwrap();
        self.items.push((ele, *price));
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices = self.items.iter().map(|el| el.1).collect::<Vec<f32>>();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let elem = prices[(self.items.len() / 3)..].to_vec();
        let perc = (elem.iter().sum::<f32>()) as f32 / prices.iter().sum::<f32>() as f32;

        self.receipt = prices
            .iter()
            .map(|pr| (pr * perc * 100.).round() / 100.)
            .collect();

        self.receipt.clone()
    }
}
