use crate::graphql::{ProductsMutation, ProductsQuery};
use async_graphql::{EmptySubscription, Schema};

pub type ProductsSchema = Schema<ProductsQuery, ProductsMutation, EmptySubscription>;
