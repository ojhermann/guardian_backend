-- Your SQL goes here
CREATE PROCEDURE delete_gurl(
    id_value INTEGER
)
LANGUAGE plpgsql
AS $$
BEGIN
    DELETE FROM gurls
    WHERE id = id_value;

    COMMIT;
END;$$
