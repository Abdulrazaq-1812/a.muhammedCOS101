--
-- PostgreSQL database dump
--

\restrict fbozRQvvwKnxMuN4hBu7tTpxMfuVEB46VEi5T69wbzHh5ZuRTurz7gmuYIf4UF2

-- Dumped from database version 18.7
-- Dumped by pg_dump version 18.7

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: employees; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.employees (
    eid integer NOT NULL,
    ename character varying(25) NOT NULL,
    dno integer NOT NULL,
    esal numeric(10,2) NOT NULL,
    age integer NOT NULL,
    phone text NOT NULL
);


ALTER TABLE public.employees OWNER TO postgres;

--
-- Data for Name: employees; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.employees (eid, ename, dno, esal, age, phone) FROM stdin;
101	Alade Joy	2	250000.00	33	08023089832
100	Mustapha Ali	3	175000.00	32	08063285831
107	Alokwe Martin	7	380000.00	48	07090082812
97	Dankade Aminat	5	550000.00	40	09023688832
108	Josiah Joshua	1	120000.00	30	08053189131
102	Mankinde Mary	2	450000.00	55	09023487830
120	Adeleke Jane	4	200000.00	38	07061045862
122	Osahon Mark	6	320000.00	44	08022289842
117	Suleman Ajayi	3	800000.00	50	07030089811
104	Kuti Lawal	1	750000.00	35	09145689842
\.


--
-- Name: employees employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.employees
    ADD CONSTRAINT employees_pkey PRIMARY KEY (eid);


--
-- PostgreSQL database dump complete
--

\unrestrict fbozRQvvwKnxMuN4hBu7tTpxMfuVEB46VEi5T69wbzHh5ZuRTurz7gmuYIf4UF2

