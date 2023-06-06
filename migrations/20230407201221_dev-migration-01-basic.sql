
-- IMS SCHEMA --

CREATE SCHEMA IF NOT EXISTS ims;

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

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
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS ims.units
(
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    short TEXT NOT NULL,
    description TEXT NULL,
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
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (location_id) REFERENCES ims.locations(id),
    FOREIGN KEY (piece_id) REFERENCES ims.pieces(id),
    FOREIGN KEY (unit) REFERENCES ims.units(id)
);


-- OMS SCHEMA --

CREATE SCHEMA IF NOT EXISTS oms;

CREATE TABLE IF NOT EXISTS oms.orders
(
    id SERIAL PRIMARY KEY,
    code UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    title TEXT NOT NULL,
    description TEXT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS oms.order_entries
(
    id SERIAL PRIMARY KEY,
    order_id INTEGER NOT NULL,
    piece_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    unit INTEGER NOT NULL,
    line_price DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (order_id) REFERENCES oms.orders(id),
    FOREIGN KEY (piece_id) REFERENCES ims.pieces(id),
    FOREIGN KEY (unit) REFERENCES ims.units(id)
);

-- Path: migrations\20230407201221_dev-migration-01-basic.sql