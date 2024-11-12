-- Active: 1731069975349@@127.0.0.1@5432@postgres
CREATE TABLE "users" (
    "id" TEXT DEFAULT gen_random_uuid (),
    "name" VARCHAR(255) NOT NULL,
    "username" VARCHAR(255) NOT NULL UNIQUE,
    "password" VARCHAR(255) NOT NULL,
    PRIMARY KEY ("id")
);