table! {
    posts (id) {
        id -> Integer,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
        deleted_at -> Nullable<Datetime>,
        title -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        body -> Nullable<Text>,
    }
}
