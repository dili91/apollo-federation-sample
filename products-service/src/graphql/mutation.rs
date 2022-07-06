use crate::repository::{CreateProductRequest, ProductsRepository, ProductsRepositoryQueries};
use async_graphql::*;

pub struct ProductsMutation;

#[Object]
impl ProductsMutation {
    async fn create_product<'a>(
        &self,
        ctx: &Context<'a>,
        sku: String,
        name: String,
        description: String,
        price_in_minor: i32,
    ) -> std::result::Result<String, Error> {
        let repo = ctx.data_unchecked::<ProductsRepository>();

        let created = repo.create_product(CreateProductRequest {
            sku,
            name,
            description,
            price_in_minor,
        }).await?;

        Ok(created.to_string())
    }
}
