-- Your SQL goes here
CREATE TABLE memo
(
    id          INTEGER NOT NULL
        CONSTRAINT memo_pk
            PRIMARY KEY autoincrement,
    content     TEXT,
    language    TEXT,
    source_type TEXT,
    result_type TEXT
);