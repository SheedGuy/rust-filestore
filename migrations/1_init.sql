-- Stolen from:
-- https://github.com/launchbadge/realworld-axum-sqlx/blob/f1b25654773228297e35c292f357d33b7121a101/migrations/1_setup.sql#L26C19-L26C19
-- with love <3

create extension if not exists "uuid-ossp";

-- Creating a trigger to update the "updated_at" column
create or replace function set_updated_at()
    returns trigger as
$$
begin
    NEW.updated_at = now();
    return NEW;
end;
$$ language plpgsql;


-- Creating a function to use AFTER creating a table to add the above trigger to said table
create or replace function trigger_updated_at(tablename regclass)
    returns void as
$$
begin
    execute format('CREATE TRIGGER set_updated_at
        BEFORE UPDATE
        ON %s
        FOR EACH ROW
        WHEN (OLD is distinct from NEW)
    EXECUTE FUNCTION set_updated_at();', tablename);
end;
$$ language plpgsql;