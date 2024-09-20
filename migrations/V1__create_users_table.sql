-- V1__create_users_table.sql

CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    telegram_user_id BIGINT NOT NULL,
    UNIQUE INDEX idx_telegram_user_id (telegram_user_id)
);
