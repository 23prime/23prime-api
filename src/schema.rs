pub mod gokabot {
    table! {
        gokabot.animes (id) {
            id -> Int4,
            year -> Int4,
            season -> Varchar,
            day -> Bpchar,
            time -> Bpchar,
            station -> Varchar,
            title -> Varchar,
            recommend -> Bool,
        }
    }

    table! {
        gokabot.cities (id) {
            id -> Int4,
            name -> Varchar,
            jp_name -> Nullable<Varchar>,
        }
    }

    table! {
        gokabot.gokabous (id) {
            id -> Int4,
            reg_date -> Date,
            sentence -> Varchar,
        }
    }

    allow_tables_to_appear_in_same_query!(animes, cities, gokabous,);
}
