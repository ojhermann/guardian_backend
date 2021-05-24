-- Your SQL goes here
CREATE OR REPLACE FUNCTION public.get_most_recently_added_gurl()
RETURNS SETOF gurl_response
AS $$
BEGIN
    RETURN QUERY
    SELECT g.id,
           g.url,
           g.created_at,
           g.liked
    FROM public.gurls as g
    ORDER BY g.id DESC
    LIMIT 1;
END;
$$
LANGUAGE plpgsql;
