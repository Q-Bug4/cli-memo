-- Your SQL goes here
CREATE TABLE memos (
  id INTEGER NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  content VARCHAR NOT NULL,
  language VARCHAR NOT NULL,
  source_type VARCHAR NOT NULL,
  result_type VARCHAR NOT NULL
)