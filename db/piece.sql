CREATE TABLE IF NOT EXISTS pieces 
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    piece_code UUID NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    category INTEGER NOT NULL,

    -- Dates strictly for tracked in PostgreSQL --
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    -- Foreign Key --
    FOREIGN KEY (category) REFERENCES piece_categories(id),

)