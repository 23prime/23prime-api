ALTER TABLE "gokabot"."animes"
DROP CONSTRAINT animes_constraints;

ALTER TABLE "gokabot"."animes"
ADD CONSTRAINT animes_constraints UNIQUE ("year", "season", "title") 
