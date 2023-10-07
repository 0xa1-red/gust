ALTER TABLE IF EXISTS gusts
    ADD COLUMN language character varying(200) not null default 'text';