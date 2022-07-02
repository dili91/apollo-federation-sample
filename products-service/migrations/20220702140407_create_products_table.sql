-- Add migration script here
-- Create Products Table
CREATE TABLE products   (
    id uuid NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    price_in_minor integer  NOT NULL
);