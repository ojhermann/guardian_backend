-- Your SQL goes here
CREATE OR REPLACE FUNCTION public.get_gurls(start_id INTEGER, end_id INTEGER)
RETURNS SETOF gurl_response
AS $$
BEGIN
    RETURN QUERY
    SELECT g.id,
           g.url,
           g.created_at,
           g.liked
    FROM public.gurls as g
    WHERE g.id >= start_id
          AND g.id < end_id;
END;
$$
LANGUAGE plpgsql;
