-- @block Init DB
-- Delete the tables if they exists
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS api_keys;
DROP TABLE IF EXISTS users;
-- Create the schema
CREATE TABLE users (
    uuid CHAR(36) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash CHAR(97) NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (uuid)
);
CREATE TABLE api_keys (
    uuid CHAR(36) NOT NULL UNIQUE,
    user_uuid CHAR(36) NOT NULL,
    api_key_hash CHAR(64) NOT NULL UNIQUE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_used_at DATETIME ON UPDATE CURRENT_TIMESTAMP,
    expires_at DATETIME DEFAULT (DATE_ADD(CURRENT_TIMESTAMP, INTERVAL 7 DAY)),
    PRIMARY KEY (uuid),
    FOREIGN KEY (user_uuid) REFERENCES users(uuid)
);
CREATE TABLE tags (
    uuid CHAR(36) NOT NULL,
    user_uuid CHAR(36) NOT NULL,
    name VARCHAR(255) NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_uuid, name),
    PRIMARY KEY (uuid),
    FOREIGN KEY (user_uuid) REFERENCES users(uuid)
);
-- Insert some mock users
INSERT INTO users (uuid, email, password_hash)
VALUES (
        'b6cea585-0dc0-4887-8247-201f164a6d6a',
        'john.doe@gmail.com',
        '$argon2id$v=19$m=19456,t=2,p=1$FMwa6Eb1swp7PpDLXToHog$9hNgeoBrX2WeoG/amPwGI/ekSAMukXawbK54b/NyiFQ'
    ),
    (
        'c8fdc92e-f72b-4fc6-b15d-ad006e063d83',
        'jane.doe@gmail.com',
        '$argon2id$v=19$m=19456,t=2,p=1$j7RU52E7TKV6gvpUkTnfqw$HS1HlbL/bx/m6ZTQqkwy8oaylH64CGMnNwkNesxTrfw'
    ),
    (
        '68a373e4-c8d7-4449-8e63-0f216a59fd0e',
        'alice.smith@gmail.com',
        '$argon2id$v=19$m=19456,t=2,p=1$byHK//s8iG2imuuhqeuGbA$+oMywATyIdqejvsojcUR0m5ZV3izsy1KRFthYvFJDwU'
    );
-- Insert some mock tags
INSERT INTO tags (uuid, user_uuid, name)
VALUES (
        "3fd8b2aa-2665-4154-937b-a412e52d9070",
        '68a373e4-c8d7-4449-8e63-0f216a59fd0e',
        'tag1'
    ),
    (
        "e443c19e-5aab-4038-9fb2-6c385b40b4b0",
        '68a373e4-c8d7-4449-8e63-0f216a59fd0e',
        'tag2'
    ),
    (
        "88e4a42e-559b-431e-b31b-8b41bf6ea70f",
        'c8fdc92e-f72b-4fc6-b15d-ad006e063d83',
        'tag3'
    );
-- Insert mock api keys
INSERT INTO api_keys (uuid, user_uuid, api_key_hash)
VALUES (
        'f1b3b3b3-1b3b-4b3b-8b3b-1b3b3b3b3b3b',
        'b6cea585-0dc0-4887-8247-201f164a6d6a',
        '6371a4a2bfe90c3209f3ab1d8665c17fdce7f5314411e7676716f3a30f4e426f'
    );