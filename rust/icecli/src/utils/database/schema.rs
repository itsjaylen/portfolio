// @generated automatically by Diesel CLI.

diesel::table! {
    current_currency_values (id) {
        id -> Nullable<Integer>,
        currency_code -> Text,
        currency_name -> Text,
        exchange_rate -> Double,
        last_updated -> Nullable<Timestamp>,
        country -> Nullable<Text>,
        symbol -> Nullable<Text>,
        market_cap -> Nullable<Double>,
        volume_24h -> Nullable<Double>,
    }
}

diesel::table! {
    historical_currency_values (id) {
        id -> Nullable<Integer>,
        currency_code -> Text,
        exchange_rate -> Double,
        recorded_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    current_currency_values,
    historical_currency_values,
);
