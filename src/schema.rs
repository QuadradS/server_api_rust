// @generated automatically by Diesel CLI.

diesel::table! {
    rustoceans (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        cteated -> Timestamp,
    }
}
