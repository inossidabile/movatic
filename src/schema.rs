// @generated automatically by Diesel CLI.

diesel::table! {
    stations (station_id) {
        station_id -> Varchar,
        name -> Nullable<Varchar>,
        address -> Nullable<Varchar>,
        latitude -> Nullable<Numeric>,
        longitude -> Nullable<Numeric>,
        is_renting -> Nullable<Bool>,
        is_returning -> Nullable<Bool>,
        is_installed -> Nullable<Bool>,
        num_docks_available -> Nullable<Int8>,
        num_bikes_available -> Nullable<Int8>,
        last_reported -> Nullable<Timestamptz>,
    }
}
