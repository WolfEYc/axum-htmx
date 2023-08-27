CREATE TABLE admin_client (
    client_id BIGINT PRIMARY KEY,
    FOREIGN KEY (client_id) REFERENCES client (id)
);