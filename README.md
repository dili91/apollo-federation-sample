# Apollo Federation Gateway sample

A sample stack to get familiar with [Apollo Federation gateway](https://www.apollographql.com/docs/federation/) features. 

## Purpose

Objective of this project is to simply get familiar with the Apollo stack and see how it behaves across completely different languages.


## The stack 

The stack is made of 2 basic sample microservices: 
- A [products-service](./products-service), that simply returns the product catalog. Based on Rust and actix-web;
- A Java based [users-service](./users-service) which is responsible of users profile informations. Built on top of Javalin

Both services expose a GraphQL interface to their data stores. 

An [federation-gateway](./federation-gateway) based on Apollo then registers the above 2 graphs as subgraphs and expose their data via the federation gateway

## Run it
//todo

## todo

- [ ] mutations
- [ ] subscriptions ? 
- [ ] authentication
- [ ] Javalin app
- [ ] federation setup
- [ ] Docker-compose and doc
- [ ] Basic CI ?
