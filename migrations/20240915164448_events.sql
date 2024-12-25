-- Add migration script here
CREATE TYPE backend.sustainable_practice AS ENUM (
    'RecycledMaterials',
    'CollectiveTransport',
    'RenewableEnergy',
    'WasteReduction'
);

CREATE TABLE IF NOT EXISTS events (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    location VARCHAR(255),
    max_participants INT DEFAULT 0,
    start_date TIMESTAMP NOT NULL,
    end_date TIMESTAMP NOT NULL,
    organizer_id INT REFERENCES backend.users(id),
    sustainable_practices backend.sustainable_practice[] NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);