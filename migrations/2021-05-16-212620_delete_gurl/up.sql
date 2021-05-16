-- Your SQL goes here
CREATE OR REPLACE FUNCTION public.delete_gurl(id_value INTEGER)
RETURNS VOID
AS $$
BEGIN
    DELETE FROM public.gurls
    WHERE id = id_value;
END;
$$
LANGUAGE plpgsql;
