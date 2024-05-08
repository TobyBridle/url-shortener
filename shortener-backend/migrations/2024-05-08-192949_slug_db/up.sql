-- The database uses the URL within the slug table as a foreign key into
-- another table containing all unique URLs.
-- Doing so allows us to not duplicate URLs over and over, especially since
-- many people may shorten the same URL (e.g of popular social media pages)
-- Since every generation is unique, so that the user can view statistics
-- specific to their generation, we can't just share the shortened link

CREATE TABLE url_db (
    Url TEXT PRIMARY KEY,
    Id BIGSERIAL NOT NULL UNIQUE
);

CREATE TABLE slug_db (
    Slug TEXT PRIMARY KEY,
    Id BIGINT NOT NULL REFERENCES url_db(Id)
);