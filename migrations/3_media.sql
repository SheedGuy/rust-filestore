CREATE TYPE media_purpose AS ENUM ('avatar');

create table "media"
(
    media_id        uuid                primary key,
    file_name       text                not null,
    content_type    text                not null,
    media_purpose   media_purpose       not null,
    created_at      timestamptz         not null default now(),
    updated_at      timestamptz         default now()
);

SELECT trigger_updated_at('"media"');