CREATE TABLE verified_client (
    client_id BIGINT PRIMARY KEY,
    FOREIGN KEY (client_id) REFERENCES client (id)
);