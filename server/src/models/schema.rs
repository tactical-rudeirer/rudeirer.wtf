table! {
    airline (id) {
        id -> Int4,
        name -> Varchar,
        iata_code -> Varchar,
        description -> Nullable<Text>,
        logo_filename -> Varchar,
    }
}

table! {
    airport (id) {
        id -> Int4,
        city_id -> Int4,
        name -> Varchar,
        iata_code -> Varchar,
        description -> Text,
        popular -> Bool,
        airport_fees -> Nullable<Float8>,
    }
}

table! {
    arenas (id) {
        id -> Int4,
        name -> Nullable<Text>,
        long -> Nullable<Float8>,
        lat -> Nullable<Float8>,
    }
}

table! {
    balance (id) {
        id -> Int4,
        user_id -> Int4,
        jens -> Numeric,
        rudicoins -> Numeric,
        leibi -> Numeric,
        rmr -> Numeric,
        bsc -> Numeric,
        rpy -> Numeric,
        business -> Bool,
    }
}

table! {
    booking (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        departure_city_id -> Int4,
        arrival_city_id -> Int4,
        booking_code -> Varchar,
        departure_date -> Timestamp,
        return_date -> Timestamp,
        outbound_route_id -> Int4,
        inbound_route_id -> Int4,
        booking_class -> Varchar,
        seats_outbound -> Text,
        seats_inbound -> Text,
        payment_method -> Varchar,
        credit_card -> Nullable<Varchar>,
        account_number -> Nullable<Varchar>,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    city (id) {
        id -> Int4,
        popular -> Bool,
        name -> Varchar,
        description -> Text,
        short_description -> Text,
        image_filename -> Varchar,
        slug -> Varchar,
        popularity -> Int4,
        magic_pricing_factor -> Nullable<Float8>,
        base_price_per_night -> Nullable<Float8>,
        timezone -> Varchar,
    }
}

table! {
    currencies (id) {
        id -> Int4,
        sym -> Text,
        name -> Nullable<Text>,
    }
}

table! {
    jenskurs (id) {
        id -> Int4,
        price -> Nullable<Numeric>,
        date -> Nullable<Timestamptz>,
        currency -> Nullable<Text>,
    }
}

table! {
    kurse (id) {
        id -> Int4,
        price -> Numeric,
        date -> Timestamptz,
        currency -> Text,
    }
}

table! {
    matches (id) {
        id -> Int4,
        home_id -> Nullable<Int4>,
        away_id -> Nullable<Int4>,
        date -> Timestamptz,
        arena_id -> Nullable<Int4>,
        resulthome -> Nullable<Int4>,
        resultaway -> Nullable<Int4>,
        referee_id -> Nullable<Int4>,
    }
}

table! {
    news (id) {
        id -> Int4,
        title -> Text,
        content -> Text,
        date -> Date,
        author_id -> Int4,
    }
}

table! {
    planned_flight (id) {
        id -> Int4,
        airport_from_id -> Int4,
        airport_to_id -> Int4,
        vessel_type_id -> Int4,
        flight_number -> Varchar,
        description -> Nullable<Text>,
        magic_pricing_factor -> Nullable<Float8>,
        travel_time_seconds -> Int4,
        boarding_time_utc -> Nullable<Time>,
        departure_time_utc -> Time,
        operating_airline_id -> Int4,
    }
}

table! {
    players (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    point_of_interest (id) {
        id -> Int4,
        city_id -> Int4,
        name -> Varchar,
        description -> Text,
        popular -> Bool,
        slug -> Varchar,
        short_description -> Text,
        teaser_image_filename -> Nullable<Varchar>,
        image_filenames -> Nullable<Text>,
    }
}

table! {
    route (id) {
        id -> Int4,
    }
}

table! {
    route_segment (id) {
        id -> Int4,
        route_id -> Nullable<Int4>,
        scheduled_flight_id -> Int4,
        segment_no -> Int4,
    }
}

table! {
    scheduled_flight (id) {
        id -> Int4,
        planned_flight_id -> Nullable<Int4>,
        vessel_id -> Int4,
        originally_scheduled_for -> Timestamp,
        boarding_time_utc -> Nullable<Timestamp>,
        departure_time_utc -> Timestamp,
        travel_time_seconds -> Int4,
    }
}

table! {
    stocks (id) {
        id -> Int4,
        sym -> Text,
        name -> Text,
        profit -> Int4,
        savings -> Int4,
        dividend -> Float8,
    }
}

table! {
    tradeoffers (id) {
        id -> Int4,
        user_id -> Int4,
        offer_currency -> Text,
        offer_amount -> Numeric,
        asking_currency -> Text,
        asking_amount -> Numeric,
        time -> Nullable<Timestamptz>,
    }
}

table! {
    transactions (id) {
        id -> Int4,
        recipient -> Int4,
        sender -> Int4,
        amount -> Numeric,
        currency -> Text,
        reference -> Text,
        time -> Timestamptz,
    }
}

table! {
    traveller (id) {
        id -> Int4,
        last_name -> Varchar,
        first_names -> Varchar,
        address -> Varchar,
        address2 -> Nullable<Varchar>,
        zip_code -> Varchar,
        city -> Varchar,
        country -> Varchar,
        booking_id -> Int4,
        traveller_no -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        displayname -> Nullable<Text>,
        role -> Nullable<Text>,
    }
}

table! {
    vessel (id) {
        id -> Int4,
        type_id -> Int4,
        name -> Nullable<Varchar>,
        number -> Nullable<Varchar>,
        comment -> Nullable<Text>,
    }
}

table! {
    vessel_type (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        seat_rows -> Int4,
        seats_per_row -> Int4,
        available_booking_classes -> Text,
        magic_pricing_factor -> Nullable<Float8>,
        floorplan_first -> Nullable<Text>,
        floorplan_business -> Nullable<Text>,
        floorplan_premium -> Nullable<Text>,
        floorplan_economy -> Nullable<Text>,
        amenities -> Nullable<Text>,
    }
}

table! {
    wasisstrudihighscore (id) {
        id -> Int4,
        name -> Text,
        score -> Int4,
        date -> Nullable<Timestamptz>,
    }
}

joinable!(airport -> city (city_id));
joinable!(balance -> users (user_id));
joinable!(booking -> users (user_id));
joinable!(matches -> arenas (arena_id));
joinable!(news -> users (author_id));
joinable!(planned_flight -> airline (operating_airline_id));
joinable!(planned_flight -> vessel_type (vessel_type_id));
joinable!(point_of_interest -> city (city_id));
joinable!(route_segment -> route (route_id));
joinable!(route_segment -> scheduled_flight (scheduled_flight_id));
joinable!(scheduled_flight -> planned_flight (planned_flight_id));
joinable!(scheduled_flight -> vessel (vessel_id));
joinable!(tradeoffers -> users (user_id));
joinable!(traveller -> booking (booking_id));
joinable!(vessel -> vessel_type (type_id));

allow_tables_to_appear_in_same_query!(
    airline,
    airport,
    arenas,
    balance,
    booking,
    city,
    currencies,
    jenskurs,
    kurse,
    matches,
    news,
    planned_flight,
    players,
    point_of_interest,
    route,
    route_segment,
    scheduled_flight,
    stocks,
    tradeoffers,
    transactions,
    traveller,
    users,
    vessel,
    vessel_type,
    wasisstrudihighscore,
);
