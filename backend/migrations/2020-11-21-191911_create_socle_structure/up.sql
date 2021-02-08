CREATE TABLE domains (
    id SERIAL PRIMARY KEY,
    rank INTEGER NOT NULL UNIQUE,
    title TEXT NOT NULL
);

CREATE TABLE components (
    id SERIAL PRIMARY KEY,
    domain_id INTEGER NOT NULL,
    rank INTEGER NOT NULL,
    title TEXT NOT NULL,
    FOREIGN KEY (domain_id) REFERENCES domains (id),
    UNIQUE(domain_id, rank)
);

CREATE TABLE competencies (
    id SERIAL PRIMARY KEY,
    component_id INTEGER NOT NULL,
    rank INTEGER NOT NULL,
    title TEXT NOT NULL,
    c1 TEXT,
    c2 TEXT,
    c3 TEXT,
    c4 TEXT,
    FOREIGN KEY (component_id) REFERENCES components (id),
    UNIQUE(component_id, rank)
);
