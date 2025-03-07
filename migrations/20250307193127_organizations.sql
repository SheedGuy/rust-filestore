create table "organizations" 
(
    org_id          uuid                primary key,
    org_name        text                not null,
    slug            text                not null,
    bucket_name     text                not null,
    created_at      timestamptz         not null default now(),
    updated_at      timestamptz         default now()
);

SELECT trigger_updated_at('"organizations"');