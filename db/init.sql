--
-- PostgreSQL database dump
--

-- Dumped from database version 14.7
-- Dumped by pg_dump version 14.7 (Homebrew)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: uuid-ossp; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;


--
-- Name: EXTENSION "uuid-ossp"; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION "uuid-ossp" IS 'generate universally unique identifiers (UUIDs)';


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: occupancies; Type: TABLE; Schema: public; Owner: zoomer
--

CREATE TABLE public.occupancies (
    id integer NOT NULL,
    occupied_room_id uuid NOT NULL,
    occupied_until timestamp with time zone NOT NULL,
    meeting_title character varying NOT NULL,
    comments character varying
);


ALTER TABLE public.occupancies OWNER TO zoomer;

--
-- Name: occupancies_id_seq; Type: SEQUENCE; Schema: public; Owner: zoomer
--

CREATE SEQUENCE public.occupancies_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.occupancies_id_seq OWNER TO zoomer;

--
-- Name: occupancies_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: zoomer
--

ALTER SEQUENCE public.occupancies_id_seq OWNED BY public.occupancies.id;


--
-- Name: rooms; Type: TABLE; Schema: public; Owner: zoomer
--

CREATE TABLE public.rooms (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    name character varying NOT NULL,
    room_id character varying NOT NULL,
    capacity integer,
    time_limit interval,
    link character varying NOT NULL,
    comments character varying
);


ALTER TABLE public.rooms OWNER TO zoomer;

--
-- Name: occupancies id; Type: DEFAULT; Schema: public; Owner: zoomer
--

ALTER TABLE ONLY public.occupancies ALTER COLUMN id SET DEFAULT nextval('public.occupancies_id_seq'::regclass);


--
-- Data for Name: occupancies; Type: TABLE DATA; Schema: public; Owner: zoomer
--

COPY public.occupancies (id, occupied_room_id, occupied_until, meeting_title, comments) FROM stdin;
\.


--
-- Data for Name: rooms; Type: TABLE DATA; Schema: public; Owner: zoomer
--

COPY public.rooms (id, name, room_id, capacity, time_limit, link, comments) FROM stdin;
\.


--
-- Name: occupancies_id_seq; Type: SEQUENCE SET; Schema: public; Owner: zoomer
--

SELECT pg_catalog.setval('public.occupancies_id_seq', 1, false);


--
-- Name: occupancies occupancies_pkey; Type: CONSTRAINT; Schema: public; Owner: zoomer
--

ALTER TABLE ONLY public.occupancies
    ADD CONSTRAINT occupancies_pkey PRIMARY KEY (id);


--
-- Name: rooms rooms_name_key; Type: CONSTRAINT; Schema: public; Owner: zoomer
--

ALTER TABLE ONLY public.rooms
    ADD CONSTRAINT rooms_name_key UNIQUE (name);


--
-- Name: rooms rooms_pkey; Type: CONSTRAINT; Schema: public; Owner: zoomer
--

ALTER TABLE ONLY public.rooms
    ADD CONSTRAINT rooms_pkey PRIMARY KEY (id);


--
-- Name: rooms rooms_room_id_key; Type: CONSTRAINT; Schema: public; Owner: zoomer
--

ALTER TABLE ONLY public.rooms
    ADD CONSTRAINT rooms_room_id_key UNIQUE (room_id);


--
-- Name: occupancies fk_room; Type: FK CONSTRAINT; Schema: public; Owner: zoomer
--

ALTER TABLE ONLY public.occupancies
    ADD CONSTRAINT fk_room FOREIGN KEY (occupied_room_id) REFERENCES public.rooms(id);


--
-- PostgreSQL database dump complete
--

