CREATE TABLE IF NOT EXISTS location_entries
(
    id SERIAL PRIMARY KEY,
    location_id INTEGER NOT NULL,
    piece_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (location_id) REFERENCES locations(id),
    FOREIGN KEY (piece_id) REFERENCES pieces(id)
)
