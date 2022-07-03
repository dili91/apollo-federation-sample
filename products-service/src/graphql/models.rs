use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Product {
    //todo: check how to use uuid
    pub id: String,
    pub sku: String,
    pub name: String,
    pub description: String,
    pub price_in_minor: i32,
}
