CREATE TABLE "stations" (
    station_id VARCHAR CONSTRAINT stations_pk PRIMARY KEY,
    name VARCHAR,
    address VARCHAR,
    latitude DECIMAL,
    longitude DECIMAL,
    is_renting BOOLEAN,
    is_returning BOOLEAN,
    is_installed BOOLEAN,
    num_docks_available INTEGER,
    num_bikes_available INTEGER,
    last_reported TIMESTAMPTZ
);