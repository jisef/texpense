CREATE TABLE IF NOT EXISTS template
(
    id_template SERIAL PRIMARY KEY,
    payment     INTEGER,
    FOREIGN KEY (payment) REFERENCES payment (id_payment),
    description TEXT

);

CREATE TABLE IF NOT EXISTS payment
(
    id_payment  SERIAL PRIMARY KEY,
    amount      DECIMAL(16, 2) NOT NULL,
    category    INTEGER,
    FOREIGN KEY (category) REFERENCES category (id_category),
    description TEXT

);
ALTER TABLE payment
ADD COLUMN label       varchar        NOT NULL;

ALTER TABLE payment
ADD COLUMN templateFK INTEGER;

Alter TABLE payment
ADD CONSTRAINT templateFK
FOREIGN KEY (templateFK) REFERENCES template(id_template);

ALTeR TABLE payment
Add COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP;



CREATE TABLE IF NOT EXISTS category
(
    id_category  SERIAL PRIMARY KEY,
    name         varchar UNIQUE NOT NULL,
    pre_category INTEGER,
    FOREIGN KEY (pre_category) REFERENCES category (id_category)
);

CREATE TABLE IF NOT EXISTS account
(
    id_account  SERIAL PRIMARY KEY,
    name        varchar UNIQUE,
    amount      DECIMAL(16, 2) DEFAULT 0,
    description TEXT
);

CREATE TABLE IF NOT EXISTS transaction
(
    id_transaction SERIAL PRIMARY KEY,
    src_account    INTEGER,
    FOREIGN KEY (src_account) REFERENCES account (id_account),
    dest_account   INTEGER,
    FOREIGN KEY (dest_account) REFERENCES account (id_account),
    description    TEXT
);