-- Add migration script here
ALTER TABLE locations
    ADD COLUMN IF NOT EXISTS image VARCHAR(255)
    CONSTRAINT image_fk REFERENCES images (filename)
    ON UPDATE CASCADE ON DELETE CASCADE;
