table! {
    article_tag_associations (article_id, tag_id) {
        article_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    articles (id) {
        id -> Int4,
        slug -> Text,
        title -> Text,
        description -> Text,
        body -> Text,
        author -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        favorites_count -> Int4,
    }
}

table! {
    comments (id) {
        id -> Int4,
        body -> Text,
        user_id -> Int4,
        article_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    favorites (user_id, article_id) {
        user_id -> Int4,
        article_id -> Int4,
    }
}

table! {
    followings (follower_id, followed_id) {
        follower_id -> Int4,
        followed_id -> Int4,
    }
}

table! {
    tags (id) {
        id -> Int4,
        tag -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        bio -> Nullable<Text>,
        image -> Nullable<Text>,
        hash -> Text,
    }
}

joinable!(article_tag_associations -> articles (article_id));
joinable!(article_tag_associations -> tags (tag_id));
joinable!(articles -> users (author));
joinable!(comments -> articles (article_id));
joinable!(comments -> users (user_id));
joinable!(favorites -> articles (article_id));
joinable!(favorites -> users (user_id));

allow_tables_to_appear_in_same_query!(
    article_tag_associations,
    articles,
    comments,
    favorites,
    followings,
    tags,
    users,
);
