-- Your SQL goes here
CREATE TYPE public.gurl_response
AS (id INTEGER,
    url TEXT,
    created_at TIMESTAMP WITH TIME ZONE,
    liked BOOLEAN);

CREATE OR REPLACE FUNCTION public.get_gurl(url_value TEXT)
RETURNS SETOF gurl_response
AS $$
BEGIN
    RETURN QUERY
    SELECT g.id,
           g.url,
           g.created_at,
           g.liked
    FROM public.gurls as g
    WHERE g.url = url_value;
END;
$$
LANGUAGE plpgsql;
