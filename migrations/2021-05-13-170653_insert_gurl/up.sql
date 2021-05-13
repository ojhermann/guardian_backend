-- Your SQL goes here
CREATE PROCEDURE insert_gurl(
    url_value TEXT,
    liked_value BOOLEAN
)
LANGUAGE plpgsql
AS $$
BEGIN
    INSERT INTO gurl (url, liked)
    VALUES (url_value, liked_value);

    COMMIT;
END;$$
