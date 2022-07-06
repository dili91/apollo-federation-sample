use crate::configuration::DatabaseSettings;
use crate::graphql;
use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;
use crate::repository::Product;

#[derive(Clone)]
pub struct ProductsRepository {
    pool: PgPool,
}

pub struct ProductsRepositoryBuilder {
    settings: DatabaseSettings,
}

impl ProductsRepositoryBuilder {
    pub fn new(settings: DatabaseSettings) -> Self {
        Self { settings }
    }

    pub async fn build(self) -> Result<ProductsRepository, Error> {
        let connection_string = self.settings.connection_string();
        let database_connection_pool = PgPool::connect(&connection_string).await?;
        Ok(ProductsRepository {
            pool: database_connection_pool,
        })
    }
}

#[async_trait]
pub trait ProductsRepositoryQueries {
    async fn list_products(&self) -> Result<Vec<Product>, Error>;

    async fn get_product_by_sku(&self, sku: &str) -> Result<Product, Error>;

    async fn insert_product(&self, ) -> Result<Product, Error>;
}

#[async_trait]
impl ProductsRepositoryQueries for ProductsRepository {
    async fn list_products(&self) -> Result<Vec<Product>, Error> {
        let products = sqlx::query_as!(Product, "SELECT * FROM products")
            .fetch_all(&self.pool)
            .await?;
        Ok(products)
    }

    async fn get_product_by_sku(&self, sku: &str) -> Result<Product, Error> {
        let product = sqlx::query_as!(Product, "SELECT * FROM products WHERE sku = $1", sku)
            .fetch_one(&self.pool)
            .await?;
        Ok(product)
    }

    async fn insert_product(&self) -> Result<Product, Error> {
        todo!()
    }
}
