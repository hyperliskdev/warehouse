
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


--- IMS SCHEMA START ---

CREATE SCHEMA IF NOT EXISTS ims;

CREATE TABLE IF NOT EXISTS ims.piece_categories 
(
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NULL,

    -- Dates strictly for tracked in PostgreSQL --
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS ims.pieces 
(
    id SERIAL PRIMARY KEY,
    code UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    description TEXT NULL,
    category INTEGER NULL,

    -- Dates strictly for tracked in PostgreSQL --
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    -- Foreign Key --
    FOREIGN KEY (category) REFERENCES ims.piece_categories(id)
);

CREATE TABLE IF NOT EXISTS ims.locations
(
    id SERIAL PRIMARY KEY,
    code UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    title TEXT NOT NULL,
    description TEXT NULL,

    -- Dates strictly for tracked in PostgreSQL --
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS ims.units
(
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    short TEXT NOT NULL,
    description TEXT NULL,

    -- Dates strictly for tracked in PostgreSQL --
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS ims.location_entries
(
    id SERIAL PRIMARY KEY,
    location_id INTEGER NOT NULL,
    piece_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    unit INTEGER NOT NULL,

    -- Dates strictly for tracked in PostgreSQL --
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    -- Foreign Key --
    FOREIGN KEY (location_id) REFERENCES ims.locations(id),
    FOREIGN KEY (piece_id) REFERENCES ims.pieces(id),
    FOREIGN KEY (unit) REFERENCES ims.units(id)
);

--- IMS SCHEMA END ---


--- EMS SCHMEA START ---

CREATE SCHEMA IF NOT EXISTS ems;

CREATE TABLE IF NOT EXISTS ems.employees
(
    id SERIAL PRIMARY KEY,
    employee_code UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    password TEXT NOT NULL,

    -- Dates strictly for tracked in PostgreSQL --
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

