-- Your SQL goes here
CREATE TABLE memos (
  id INT PRIMARY KEY,
  name VARCHAR NOT NULL,
  content VARCHAR NOT NULL,
  language VARCHAR NOT NULL,
  source_type VARCHAR NOT NULL,
  result_type VARCHAR NOT NULL
)