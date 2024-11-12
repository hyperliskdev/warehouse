-- Add migration script here
-- This table will hold all the metadata information of the model.
CREATE TABLE model_metadata (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    version VARCHAR(50) NOT NULL,
    s3_key VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT now(),
);