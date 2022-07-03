# Products service

A Rust simple app that mimics a hypothetical Products microservice.

Exposes the following endpoints: 
- `GET /up` : health check endpoint
- `GET /products`: shows products catalog
- `GET /producs/{sku}`: gets a product detail by its sku
- `POST /graphql/api`: exposes a GraphQL compliant API
- `GET /graphql/ui`: exposes a [GraphQL Apollo playground](https://www.apollographql.com/docs/apollo-server/v2/testing/graphql-playground/)


## Test

```shell
cargo test
```

## Run

```shell
cargo run
```

