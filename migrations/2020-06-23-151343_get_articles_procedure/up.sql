CREATE OR REPLACE FUNCTION get_articles (a_limit INTEGER, a_offset INTEGER, maybe_user_id INTEGER = NULL) 
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
DECLARE 
	follow_q TEXT = '';
	favorite_q TEXT = '';
	fav_result TEXT = 'false';
BEGIN

if maybe_user_id is not null then
	follow_q := 'and followings.follower_id = ' || maybe_user_id;
	favorite_q := 'and favorites.user_id = ' || maybe_user_id;
	fav_result := 'count(followings) > 0';
end if;

RETURN QUERY EXECUTE
'select articles.slug,
		articles.title,
		articles.description,
		articles.body,
		articles.created_at,
		articles.updated_at,
		users.username, 
		users.bio,
		users.image,
		array_agg(tags.tag) FILTER (WHERE tags.tag is not null),
		' || fav_result  || ' as fav_count, 
		count(followings) > 0 as fol_count,
        articles.favorites_count
from articles
inner join users on users.id = articles.author
left join article_tag_associations as atas on atas.article_id = articles.id
left join tags on atas.tag_id = tags.id
left join favorites on favorites.article_id = articles.id ' || favorite_q || 
'left join followings on followings.followed_id = articles.author ' || follow_q ||
'group by articles.id, users.id
ORDER BY articles.created_at DESC
LIMIT ' || a_limit || 
'OFFSET ' || a_offset || ';';

END; 
$$ LANGUAGE 'plpgsql';