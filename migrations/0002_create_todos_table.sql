-- Add migration script here
CREATE TABLE IF NOT EXISTS todos (
    id          BIGINT SIGNED PRIMARY KEY AUTO_INCREMENT,
    description       VARCHAR(255) NOT NULL,
    done        BOOLEAN NOT NULL DEFAULT false,
    user_id     BIGINT SIGNED NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);