CREATE TABLE eleves (
    id SERIAL PRIMARY KEY,
    firstname TEXT NOT NULL,
    lastname TEXT NOT NULL,
    birthdate DATE NOT NULL,
    school_entry DATE NOT NULL,
    cycle TEXT CHECK(cycle IN ('C1', 'C2', 'C3', 'C4')) NOT NULL,
    active BOOLEAN NOT NULL
);
