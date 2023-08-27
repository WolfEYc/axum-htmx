CREATE TABLE item (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    price REAL,
    description VARCHAR(200)
);