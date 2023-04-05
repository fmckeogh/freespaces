-- Add migration script here
CREATE TABLE IF NOT EXISTS occupancies (
    location VARCHAR(255),
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    occupancy occupancy_level NOT NULL,
    FOREIGN KEY(location) REFERENCES locations(name) ON DELETE CASCADE ON UPDATE CASCADE,
    PRIMARY KEY(location, timestamp)
);
