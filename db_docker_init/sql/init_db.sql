drop schema if exists bioenpro4to cascade ;
create schema bioenpro4to;

drop table if exists bioenpro4to.scales cascade;
drop table if exists bioenpro4to.biocells cascade;
drop table if exists bioenpro4to.trucks cascade;
drop table if exists bioenpro4to.actors cascade;
drop table if exists bioenpro4to.categories cascade;
drop table if exists bioenpro4to.users cascade;
drop table if exists bioenpro4to.roles cascade;

create table bioenpro4to.roles
(
    id   serial       not null,
    role varchar(200) not null,
    primary key (id)
);

create unique index roles_role_uindex
    on bioenpro4to.roles (role);


create table bioenpro4to.users
(
    id         varchar(200) not null,
    psw        varchar(200) not null,
    email      varchar(200) not null,
    first_name varchar(200) not null,
    last_name  varchar(200) not null,
    address    varchar(200) not null,
    fiscal_code  varchar(16) not null,
    phone_number varchar(200) not null,
    did        varchar(200) not null,
    role       serial       not null,
    primary key(id),
    FOREIGN KEY(role) REFERENCES bioenpro4to.roles(id)
);

create table bioenpro4to.categories
(
    id       serial       not null,
    category varchar(200) not null,
    primary key (id)
);

create unique index categories_category_uindex
    on bioenpro4to.categories (category);

create table bioenpro4to.actors
(
    id         varchar(200) not null,
    psw        varchar(200) not null,
    did        varchar(200) not null,
    category   serial       not null,
    primary key(id),
    FOREIGN KEY(category) REFERENCES bioenpro4to.categories
);

create table bioenpro4to.trucks
(
    plate  varchar(200) not null,
    driver varchar(200) not null,
    primary key(plate),
    FOREIGN KEY(plate) REFERENCES bioenpro4to.actors(id),
    FOREIGN KEY(driver) REFERENCES bioenpro4to.users(id)
);

create table bioenpro4to.scales
(
    plant varchar(200) not null,
    primary key(plant),
    FOREIGN KEY(plant) REFERENCES bioenpro4to.actors(id)
);

create table bioenpro4to.biocells
(
    digestor_id           varchar(200) not null,
    plant        varchar(200) not null,
    max_capacity integer      not null,
    primary key(digestor_id),
    FOREIGN KEY(digestor_id) REFERENCES bioenpro4to.actors(id)
);

INSERT INTO bioenpro4to.roles (id, role) VALUES
    (0, 'driver'),
    (1, 'generic');

INSERT INTO bioenpro4to.categories (id, category) VALUES
    (0, 'trucks'),
    (1, 'scales'),
    (2, 'biocells');

INSERT INTO bioenpro4to.users (id, psw, email, first_name, last_name, address, fiscal_code, phone_number, did, role) VALUES
    ('m111', 'f91bad83ce31d38aa8fab39fbc3789b825bd67814f7b9994cdc3d062acfe6b34', 'p.rossi@reply.it', 'Paolo', 'Rossi', 'Corso Galileo, 123', 'RSSPLA88A01L219T', '333754786', 'did:iota:test:dF7ET9vQGs1S5RDaVfCqcH6gc4coZ35mxJVZABrh2o2', 0),
    ('m222', 'f91bad83ce31d38aa8fab39fbc3789b825bd67814f7b9994cdc3d062acfe6b34', 'f.neri@reply.it', 'Francesco', 'Neri', 'Corso Mediterraneo, 18', 'NREFNC69H01L219W', '3317584932', 'did:iota:test:7M3jibZDJXcVWFZwkewnVm2nRx3gJWkGLzJkY4Ho6o2K', 1);

INSERT INTO bioenpro4to.actors (id, psw, did, category) VALUES
    ('aa000aa', 'f91bad83ce31d38aa8fab39fbc3789b825bd67814f7b9994cdc3d062acfe6b34', 'did:iota:test:2GxszDLXHWs4kJgwbz1ch5Gwh4MBBwZxJyaU3Yf6bHij', 0),
    ('CIDIU', 'f91bad83ce31d38aa8fab39fbc3789b825bd67814f7b9994cdc3d062acfe6b34', 'did:iota:test:H7PfugwjWzbexFES2t54tfEi2dmtQ9qEw6UZA8rPhmhg', 1),
    ('d111', 'f91bad83ce31d38aa8fab39fbc3789b825bd67814f7b9994cdc3d062acfe6b34', 'did:iota:test:BG6DuW2ESTyvLR2CJA4GJAT53NfMJohZYjmfWRiGySeg', 2);

INSERT INTO bioenpro4to.trucks(plate, driver) VALUES ('aa000aa', 'm111');

INSERT INTO bioenpro4to.scales (plant) VALUES ('CIDIU');

INSERT INTO bioenpro4to.biocells (digestor_id, plant, max_capacity) VALUES ('d111', 'CIDIU Druento', 2000);

