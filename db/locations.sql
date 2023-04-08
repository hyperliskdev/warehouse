CREATE TABLE IF NOT EXISTS locations
(
    id SERIAL PRIMARY KEY,
    location_code UUID NOT NULL,
    title TEXT NOT NULL,
    description TEXT NULL,

    -- Dates strictly for tracked in PostgreSQL --
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)