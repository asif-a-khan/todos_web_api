CREATE TABLE IF NOT EXISTS users (
    id                      BIGINT SIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    username                VARCHAR(255) UNIQUE NOT NULL,
    password_hash           VARCHAR(255) NOT NULL,
    email                   VARCHAR(255) UNIQUE,
    phone_number            VARCHAR(20),
    phone_number_verified   BOOLEAN NOT NULL DEFAULT false,
    created_at              TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at              TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);