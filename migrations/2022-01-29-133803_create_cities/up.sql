CREATE TABLE "gokabot"."cities" (
    "id" int4 PRIMARY KEY,
    "name" varchar(300) NOT NULL,
    "jp_name" varchar(300) NULL
);

CREATE INDEX "cities_jp_name_idx" ON "gokabot"."cities" ("jp_name");
CREATE INDEX "cities_name_idx" ON "gokabot"."cities" ("name");
