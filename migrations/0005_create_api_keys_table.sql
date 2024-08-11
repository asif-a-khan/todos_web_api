-- Add migration script here
CREATE TABLE api_keys (
    id              BIGINT SIGNED PRIMARY KEY AUTO_INCREMENT,
    api_key         VARCHAR(255) UNIQUE NOT NULL,
    client_name     VARCHAR(255) NOT NULL,
    contact_email   VARCHAR(255) UNIQUE NOT NULL,
    is_active       BOOLEAN NOT NULL DEFAULT false, 
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP 
);