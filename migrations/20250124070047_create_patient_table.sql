CREATE TYPE gender AS ENUM ('male', 'female');

CREATE TABLE patients
(
    id          uuid        NOT NULL,
    PRIMARY KEY (id),
    family      text        NOT NULL,
    given       text[]      NULL,
    gender      gender      NULL,
    birth_date  timestamptz NOT NULL,
    active      boolean     NOT NULL,
    version     timestamptz NOT NULL
);
