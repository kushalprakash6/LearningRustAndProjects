use fake::Dummy;

/// Category of products our comapany sells
#[derive(Debug, Dummy)]
pub enum ProductCategory {
    Ladder, 
    Hammer,
}

#[derive(Debug, Dummy)]
pub struct Item {
    pub name: String,
    pub category: ProductCategory,
    pub quantity: u32,
}

impl Item {
    pub fn new (name: String, category:ProductCategory, quantity: u32) -> Self {
        super::talk_to_manager();
        Self {
            name,
            category,
            quantity,
        }
    }
}