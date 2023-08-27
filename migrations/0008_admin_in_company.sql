CREATE TABLE admin_in_company (
    company_id INT,
    client_id BIGINT,
    PRIMARY KEY (company_id, client_id),
    FOREIGN KEY (company_id) REFERENCES company(id),
    FOREIGN KEY (client_id) REFERENCES client(id)
);