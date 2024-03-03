-- public.user definition

CREATE TABLE IF NOT EXISTS public.user (
	id uuid NOT NULL default gen_random_uuid(),
	email varchar(25) NOT NULL,
  firstname varchar,
  lastname varchar,
  created_at timestamp default current_timestamp,
  updated_at timestamp default current_timestamp,
  is_active boolean default true,
  PRIMARY KEY (id)
);

CREATE UNIQUE INDEX idx_user_email ON public.user(email);
