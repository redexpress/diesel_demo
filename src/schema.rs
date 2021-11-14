table! {
    animals (id) {
        id -> Int4,
        species -> Varchar,
        legs -> Int4,
        name -> Nullable<Varchar>,
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
    comments (id) {
        id -> Int4,
        blog_id -> Int4,
        body -> Varchar,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(blogs -> users (user_id));
joinable!(comments -> blogs (blog_id));

allow_tables_to_appear_in_same_query!(
    animals,
    blogs,
    comments,
    posts,
    users,
);
