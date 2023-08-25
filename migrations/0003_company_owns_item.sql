CREATE TABLE company_owns_item (
    company_id INT,
    item_id BIGINT,
    amount BIGINT NOT NULL,
    PRIMARY KEY (company_id, item_id),
    FOREIGN KEY (company_id) REFERENCES company(id),
    FOREIGN KEY (item_id) REFERENCES item(id)
);