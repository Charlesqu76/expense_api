CREATE TABLE
    category (
        id SERIAL PRIMARY KEY,
        name varchar(50) UNIQUE,
        icon varchar(50)
    );

CREATE TABLE
    expense (
        id SERIAL PRIMARY KEY,
        amount float NOT NULL,
        description text,
        date timestamp NOT NULL,
        category_id NOT NULL FOREIGN KEY REFERENCES category (id)
    )