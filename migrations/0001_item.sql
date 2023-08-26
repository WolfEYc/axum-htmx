CREATE TABLE item (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    price REAL,
    description VARCHAR
);