-- Table: public.gusts

CREATE TABLE IF NOT EXISTS public.gusts
(
    key character varying(200) COLLATE pg_catalog."default" NOT NULL,
    content bytea NOT NULL,
    expires timestamp without time zone,
    anonymous bool default false,
	is_volatile bool default false,
    reads_remaining integer DEFAULT -1,
    visibility character varying(8) NOT NULL DEFAULT 'public',
    created_at timestamp without time zone NOT NULL DEFAULT now(),
	accessed integer DEFAULT 0,
	starred integer DEFAULT 0,
    CONSTRAINT gusts_pkey PRIMARY KEY (key)
);

ALTER TABLE IF EXISTS public.gusts
    OWNER to postgres;