CREATE TABLE "gokabot"."animes" (
    "id" SERIAL PRIMARY KEY,
    "year" int4 NOT NULL,
    "season" varchar(6) NOT NULL,
    "day" bpchar(3) NOT NULL,
    "time" bpchar(5) NOT NULL,
    "station" varchar(20) NOT NULL,
    "title" varchar(100) NOT NULL,
    "recommend" bool NOT NULL,
    CONSTRAINT animes_constraints UNIQUE ("year", "season", "day", "time", "station", "title", "recommend")
);

CREATE TABLE "gokabot"."gokabous" (
    "id" SERIAL PRIMARY KEY,
    "reg_date" date NOT NULL,
    "sentence" varchar(300) NOT NULL
);
