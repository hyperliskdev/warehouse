CREATE SCHEMA IF NOT EXISTS ims;

CREATE TABLE IF NOT EXISTS ims.piece_categories 
(
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL

    -- Dates strictly for tracked in PostgreSQL --
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS ims.pieces 
(
    id SERIAL PRIMARY KEY,
    piece_code UUID NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    category INTEGER NOT NULL,

    -- Dates strictly for tracked in PostgreSQL --
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    -- Foreign Key --
    FOREIGN KEY (category) REFERENCES ims.piece_categories(id)
);

CREATE TABLE IF NOT EXISTS ims.locations
(
    id SERIAL PRIMARY KEY,
    location_code UUID NOT NULL,
    title TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS ims.location_entries
(
    id SERIAL PRIMARY KEY,
    location_id INTEGER NOT NULL,
    piece_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (location_id) REFERENCES ims.locations(id),
    FOREIGN KEY (piece_id) REFERENCES ims.pieces(id)
);

-- Path: migrations\20230407201221_dev-migration-01-basic.sql