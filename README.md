# Apollo Federation Gateway sample

A sample stack to get familiar with [Apollo Federation gateway](https://www.apollographql.com/docs/federation/) features. 

## The stack 

The stack is made of 2 basic sample microservices: 
- A Rust based products-service, that simply returns the product catalog
- A Java based users-service which is responsible of users profile informations

Both services expose a GraphQL interface to their data stores. 

An apollo gateway app then registers the above 2 graphs as subgraphs and expose their data via the federation gateway

## Purpose

Objective of this project is to simply get familiar with the Apollo stack and see how it behaves across completely different stacks.
