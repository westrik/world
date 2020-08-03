CREATE OR REPLACE FUNCTION notify_job_updated ()
    RETURNS TRIGGER
    LANGUAGE plpgsql
AS $$
DECLARE
  channel text := TG_ARGV[0];
BEGIN
  PERFORM (
     SELECT pg_notify(channel, 'Created job')
  );
  RETURN NULL;
END;
$$;

CREATE TRIGGER notify_on_job_update
    AFTER INSERT ON jobs
    FOR EACH ROW
    EXECUTE PROCEDURE notify_job_updated('job_updates');
