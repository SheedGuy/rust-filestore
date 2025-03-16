create table "users"
(
    user_id         uuid                primary key,
    f_name          text                not null,
    l_name          text                not null,
    email           text                not null,
    avatar_id       uuid                references media,
    org_id          uuid                references organizations,
    created_at      timestamptz         not null default now(),
    updated_at      timestamptz         default now()
);

SELECT trigger_updated_at('"users"');