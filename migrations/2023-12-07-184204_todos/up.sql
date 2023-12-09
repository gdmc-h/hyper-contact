-- Your SQL goes here
CREATE TABLE todos (
    id PRIMARY KEY,
    title VARCHAR NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE
)
