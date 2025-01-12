-- @block Init DB
-- Delete the tables if they exists
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS users;
-- Create the schema
CREATE TABLE users (
    `id` INT NOT NULL AUTO_INCREMENT,
    `email` VARCHAR(255) NOT NULL,
    `password` VARCHAR(255) NOT NULL,
    `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
);
CREATE TABLE tags (
    `id` INT NOT NULL AUTO_INCREMENT,
    `user_id` INT NOT NULL,
    `name` VARCHAR(255) NOT NULL,
    `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    FOREIGN KEY (`user_id`) REFERENCES `users`(`id`)
);
-- Insert some mock data
INSERT INTO `users` (`email`, `password`)
VALUES ('john.doe@gmail.com', 'randompassword1'),
    ('luke.warm@hotmail.fr', 'randompassword2');
INSERT INTO `tags` (`user_id`, `name`)
VALUES (1, 'tag1'),
    (1, 'tag2'),
    (2, 'tag3');