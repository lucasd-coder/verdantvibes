-- Add migration script here
CREATE TYPE backend.role AS ENUM (
    'Organizer',
    'Participant'
);

CREATE TABLE IF NOT EXISTS backend.users (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    roles backend.role[] NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);