CREATE TABLE verified_company (
    company_id INT PRIMARY KEY,
    FOREIGN KEY (company_id) REFERENCES company (id)
);