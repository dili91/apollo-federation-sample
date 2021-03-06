-- Add migration script here
-- Create Products Table
CREATE TABLE products   (
    id uuid NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    sku varchar(10) NOT NULL UNIQUE,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    price_in_minor integer  NOT NULL
);