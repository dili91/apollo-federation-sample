use crate::graphql::Product;
use crate::repository::{ProductsRepository, ProductsRepositoryQueries};
use async_graphql::*;

pub struct ProductsQuery;

#[Object]
impl ProductsQuery {
    async fn list_products<'a>(&self, ctx: &Context<'a>) -> Result<Vec<Product>> {
        let repo = ctx.data_unchecked::<ProductsRepository>();
        let products = repo.list_products().await?;

        Ok(products.into_iter().map(|p| p.into()).collect())
    }

    async fn get_product_by_sku<'a>(&self, ctx: &Context<'a>, sku: String) -> Result<Product> {
        let repo = ctx.data_unchecked::<ProductsRepository>();
        let product = repo.get_product_by_sku(sku.as_str()).await?;

        Ok(product.into())
    }
}
