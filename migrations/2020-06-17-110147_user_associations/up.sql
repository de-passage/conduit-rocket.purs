-- Your SQL goes here

CREATE TABLE favorites(
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    article_id INTEGER NOT NULL REFERENCES articles(id) ON DELETE CASCADE,
    CONSTRAINT favorites_pk PRIMARY KEY (user_id, article_id)
);

CREATE TABLE followings(
    follower_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    followed_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT followings_pk PRIMARY KEY (follower_id, followed_id)
);

