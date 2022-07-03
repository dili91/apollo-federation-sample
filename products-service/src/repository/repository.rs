use crate::configuration::DatabaseSettings;
use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price_in_minor: i32,
}

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
}

#[async_trait]
impl ProductsRepositoryQueries for ProductsRepository {
    async fn list_products(&self) -> Result<Vec<Product>, Error> {
        let products = sqlx::query_as!(Product, "SELECT * FROM products")
            .fetch_all(&self.pool)
            .await?;
        Ok(products)
    }
}
