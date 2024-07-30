CREATE TABLE IF NOT EXISTS users (
    id                      BIGINT SIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    username                VARCHAR(255) UNIQUE NOT NULL,
    password_hash           VARCHAR(255) NOT NULL,
    email                   VARCHAR(255) UNIQUE,
    phone_number            VARCHAR(20),
    refresh_token           VARCHAR(255),
    refresh_token_expiry    TIMESTAMP,
    phone_number_verified   BOOLEAN NOT NULL DEFAULT false 
);