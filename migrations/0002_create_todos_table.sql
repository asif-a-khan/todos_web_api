CREATE TABLE IF NOT EXISTS todos (
    id              BIGINT SIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    user_id         BIGINT SIGNED NOT NULL,
    description     VARCHAR(255) NOT NULL,
    done            BOOLEAN NOT NULL DEFAULT false,
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY     (user_id) REFERENCES users(id)
);