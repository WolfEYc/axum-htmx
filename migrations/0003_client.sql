CREATE TABLE client (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(64) NOT NULL UNIQUE,
    otp_b32 VARCHAR(64) NOT NULL
);