-- Your SQL goes here
create table
  "public"."infra" (
    "name" VARCHAR(255) not null,
    "infra_modifier" FLOAT4 null,
    "price" INT null,
    constraint "infra_pkey" primary key ("name")
  )