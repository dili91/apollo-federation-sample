use crate::configuration::DatabaseSettings;
use crate::graphql;
use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

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
}
