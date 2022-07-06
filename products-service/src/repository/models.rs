use uuid::Uuid;
use crate::graphql;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: String,
    pub price_in_minor: i32,
}

//todo: check. Having this conversion here is meh
impl From<Product> for graphql::Product {
    fn from(p: Product) -> Self {
        graphql::Product {
            id: p.id.to_string(),
            sku: p.sku,
            name: p.name,
            description: p.description,
            price_in_minor: p.price_in_minor,
        }
    }
}