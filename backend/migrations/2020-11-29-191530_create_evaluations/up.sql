-- key store to store at least 'evaluation_date'
CREATE TABLE key_store (
    name TEXT PRIMARY KEY NOT NULL UNIQUE,
    value TEXT
);

-- evaluate an eleve for a competency
CREATE TABLE evaluations (
    id SERIAL PRIMARY KEY,
    eleve_id INTEGER NOT NULL,
    competency_id INTEGER NOT NULL,
    status TEXT CHECK(status IN ('Empty', 'InProgress', 'Acquired', 'NotAcquired')) NOT NULL,
    comment TEXT,
    FOREIGN KEY (competency_id) REFERENCES competencies (id),
    FOREIGN KEY (eleve_id) REFERENCES eleves (id),
    UNIQUE(eleve_id, competency_id)
);

CREATE TABLE general_comments (
  id SERIAL PRIMARY KEY,
  eleve_id INTEGER UNIQUE NOT NULL,
  comment TEXT,
  FOREIGN KEY (eleve_id) REFERENCES eleves (id)
)
