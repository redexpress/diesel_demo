table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    animals (id) {
        id -> Int4,
        species -> Varchar,
        legs -> Int4,
        name -> Nullable<Varchar>,
    }
}

table! {
    comments (id) {
        id -> Int4,
        post_id -> Int4,
        body -> Varchar,
    }
}

table! {
    blogs (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(blogs -> users (user_id));

allow_tables_to_appear_in_same_query!(animals, comments, blogs, users,);


