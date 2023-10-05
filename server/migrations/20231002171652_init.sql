-- create database gamac;

create table software(
    "uuid" uuid unique primary key default gen_random_uuid(),
    long_name text not null unique,
    short_name text not null,
    developer text,
    web_site text
);

create table system(
   "uuid" uuid unique primary key default gen_random_uuid(),
   os_name text not null,
   web_site text
);


create table package_manger(
   "uuid" uuid unique primary key default gen_random_uuid(),
   system uuid references system( uuid ),
   command text not null,
   install_instruction text not null ,
   update_instruction text not null ,
   delete_instruction text not null
);


create table packages(
    "uuid" uuid unique primary key default gen_random_uuid(),
    package_name text not null ,
    software uuid references software( uuid ),
    package_manger uuid references package_manger( uuid )
);
