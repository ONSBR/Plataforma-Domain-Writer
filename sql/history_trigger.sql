CREATE OR REPLACE FUNCTION save_history()
RETURNS trigger AS
$BODY$
BEGIN 
	EXECUTE format('INSERT INTO %I_history (origin_id, data) VALUES ($1.id, to_jsonb($1) - ''id'')', TG_TABLE_NAME) USING OLD;
	RETURN NEW;
END;   
$BODY$
LANGUAGE plpgsql VOLATILE
COST 100;


CREATE TRIGGER save_history
    BEFORE UPDATE ON {table}
    FOR EACH ROW
    EXECUTE PROCEDURE save_foo_history();
