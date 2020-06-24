-- Your SQL goes here
CREATE OR REPLACE FUNCTION get_comments(a_article_slug TEXT, a_limit INTEGER, a_offset INTEGER, m_user_id INTEGER = NULL)
RETURNS TABLE (
    comment_body TEXT,
    comment_creation TIMESTAMP WITH TIME ZONE,
    comment_update TIMESTAMP WITH TIME ZONE,
    author_username TEXT,
    author_bio TEXT,
    author_image TEXT,
    is_followed BOOL,
    total_comments BIGINT
) AS $$ 
BEGIN
RETURN QUERY 
    SELECT 
        comments.body,
        comments.created_at,
        comments.updated_at,
        users.username,
        users.bio,
        users.image,
        count(followings) > 0,
        count(comments.id)
    FROM comments
    INNER JOIN articles ON articles.id = comments.article_id
    INNER JOIN users ON comments.user_id = users.id
    LEFT JOIN followings ON followings.followed_id = users.id AND followings.follower_id = m_user_id
    WHERE articles.slug = a_article_slug
    GROUP BY comments.body, comments.created_at, comments.updated_at, users.username, users.bio, users.image
    ORDER BY comments.created_at DESC;
END;
$$ LANGUAGE 'plpgsql'