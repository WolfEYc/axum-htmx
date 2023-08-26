CREATE TABLE temp_client (
    temp_id SERIAL PRIMARY KEY,
    expiration TIMESTAMP DEFAULT CURRENT_TIMESTAMP + interval '15 minutes' NOT NULL,
    access SMALLINT NOT NULL,
    email VARCHAR(50) NOT NULL,
    hashword VARCHAR(128) NOT NULL
);