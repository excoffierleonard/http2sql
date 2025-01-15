-- @block Init DB
-- Delete the tables if they exists
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS users;
-- Create the schema
CREATE TABLE users (
    `id` INT NOT NULL AUTO_INCREMENT,
    `email` VARCHAR(255) NOT NULL UNIQUE,
    `password` CHAR(97) NOT NULL,
    `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
);
CREATE TABLE tags (
    `id` INT NOT NULL AUTO_INCREMENT,
    `user_id` INT NOT NULL,
    `name` VARCHAR(255) NOT NULL,
    `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (`user_id`, `name`),
    PRIMARY KEY (`id`),
    FOREIGN KEY (`user_id`) REFERENCES `users`(`id`)
);
-- Insert some mock data
INSERT INTO `users` (`email`, `password`)
VALUES (
        'john.doe@gmail.com',
        '$argon2id$v=19$m=19456,t=2,p=1$FMwa6Eb1swp7PpDLXToHog$9hNgeoBrX2WeoG/amPwGI/ekSAMukXawbK54b/NyiFQ'
    ),
    (
        'jane.doe@gmail.com',
        '$argon2id$v=19$m=19456,t=2,p=1$j7RU52E7TKV6gvpUkTnfqw$HS1HlbL/bx/m6ZTQqkwy8oaylH64CGMnNwkNesxTrfw'
    ),
    (
        'alice.smith@gmail.com',
        '$argon2id$v=19$m=19456,t=2,p=1$byHK//s8iG2imuuhqeuGbA$+oMywATyIdqejvsojcUR0m5ZV3izsy1KRFthYvFJDwU'
    );
INSERT INTO `tags` (`user_id`, `name`)
VALUES (1, 'tag1'),
    (1, 'tag2'),
    (2, 'tag3');