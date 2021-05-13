table! {
    gurls (url, created_at) {
        id -> Int4,
        url -> Text,
        created_at -> Timestamptz,
        liked -> Bool,
    }
}
