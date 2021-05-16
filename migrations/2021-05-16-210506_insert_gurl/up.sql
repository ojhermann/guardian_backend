-- Your SQL goes here
CREATE OR REPLACE FUNCTION public.insert_gurl(url_value TEXT, liked BOOLEAN)
RETURNS VOID
AS $$
BEGIN
    INSERT INTO public.gurls (url, liked)
    VALUES (url_value, liked);
END;
$$
LANGUAGE plpgsql;
