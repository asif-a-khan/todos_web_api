-- Add migration script here
CREATE TABLE IF NOT EXISTS todos (
    id              BIGINT SIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    user_id         BIGINT SIGNED NOT NULL,
    description     VARCHAR(255) NOT NULL,
    done            BOOLEAN NOT NULL DEFAULT false,
    FOREIGN KEY     (user_id) REFERENCES users(id)
);