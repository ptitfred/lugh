table! {
    sessions {
        id -> Integer,
        token -> Text,
        user_id -> Integer,
        created_at -> Text,
        expired_at -> Text,
    }
}

table! {
    settings {
        id -> Integer,
        key -> Text,
        value -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

table! {
    translations {
        id -> Integer,
        key -> Text,
        locale -> Text,
        content -> Nullable<Text>,
        created_at -> Text,
        deleted_at -> Nullable<Text>,
        user_id -> Nullable<Integer>,
        validator_id -> Nullable<Integer>,
        validated_at -> Nullable<Text>,
    }
}

table! {
    users {
        id -> Integer,
        email -> Text,
        hashed_password -> Text,
        created_at -> Text,
    }
}
