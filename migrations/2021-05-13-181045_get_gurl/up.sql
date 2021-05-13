-- Your SQL goes here
CREATE PROCEDURE get_gurl(
    url_value TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    SELECT g.id,
           g.url,
           g.create_at,
           g.liked
    FROM gurl as g
    WHERE g.url = url_value;

    COMMIT;
END;$$
