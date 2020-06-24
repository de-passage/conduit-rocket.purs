-- Your SQL goes here
CREATE OR REPLACE FUNCTION user_feed(feed_user_id INTEGER, a_limit INTEGER, a_offset INTEGER)
RETURNS TABLE (
	article_slug TEXT,
	article_title TEXT,
	article_description TEXT,
	article_body TEXT,
	article_creation TIMESTAMP WITH TIME ZONE,
	article_update TIMESTAMP WITH TIME ZONE,
	author_username TEXT,
	author_bio TEXT,
	author_image TEXT,
	tags TEXT[],
	is_favorite BOOL,
	is_followed BOOL ,
    favorites_count INTEGER
)
AS $$
BEGIN
RETURN QUERY 
select articles.slug as article_slug,
		articles.title as article_title,
		articles.description as article_description,
		articles.body as article_body,
		articles.created_at as article_creation,
		articles.updated_at as article_update,
		users.username as author_username, 
		users.bio as author_bio,
		users.image as author_image,
		array_agg(tags.tag) FILTER (WHERE tags.tag is not null) as tags,
		count(favorites.user_id) > 0 as is_favorite, 
		count(followings) > 0 as is_followed,
        articles.favorites_count as favorites_count
	from articles
	inner join users on users.id = articles.author
	left join article_tag_associations as atas on atas.article_id = articles.id
	left join tags on atas.tag_id = tags.id
	left join favorites on favorites.article_id = articles.id and favorites.user_id = feed_user_id
	inner join followings on followings.followed_id = articles.author and followings.follower_id = feed_user_id
	group by articles.id, users.id;
END; 
$$ LANGUAGE 'plpgsql';
