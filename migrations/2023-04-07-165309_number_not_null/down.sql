-- This file should undo anything in `up.sql`
DROP TABLE meows;
CREATE TABLE meows (
    number BIGINT,
    blog TEXT PRIMARY KEY
);