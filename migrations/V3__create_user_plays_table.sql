CREATE TABLE user_plays
(
    id        INT AUTO_INCREMENT PRIMARY KEY,
    user_id   INT      NOT NULL,
    noun_id   INT      NOT NULL,
    answer    BOOLEAN  NULL,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (noun_id) REFERENCES nouns (id)
);