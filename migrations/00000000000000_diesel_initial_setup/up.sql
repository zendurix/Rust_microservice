-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.



CREATE TABLE IF NOT EXISTS book (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR NOT NULL,
    author_name     VARCHAR NOT NULL
    )
    