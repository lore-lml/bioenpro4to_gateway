drop schema if exists bioenpro4to cascade ;
create schema bioenpro4to;

drop table if exists bioenpro4to.scales cascade;
drop table if exists bioenpro4to.biocells cascade;
drop table if exists bioenpro4to.trucks cascade;
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
    did        varchar(200),
    role       serial       not null,
    primary key(id),
    FOREIGN KEY(role)
        REFERENCES bioenpro4to.roles(id)
);

create table bioenpro4to.trucks
(
    plate  varchar(200) not null,
    did    varchar(200) not null,
    driver varchar(200) not null,
    primary key(plate),
    FOREIGN KEY(driver)
        REFERENCES bioenpro4to.users(id)
);

create table bioenpro4to.scales
(
    plant varchar(200) not null,
    did   varchar(200) not null,
    primary key(plant)
);

create table bioenpro4to.biocells
(
    id           varchar(200) not null,
    plant        varchar(200) not null,
    max_capacity integer      not null,
    primary key(id)
);

INSERT INTO bioenpro4to.roles (id, role) VALUES
    (1, 'driver'),
    (2, 'generic');

INSERT INTO bioenpro4to.users (id, psw, email, first_name, last_name, did, role) VALUES
    ('m111', '8a37ee8bd19d25b539e3b1b85cb3533ab6d8a691ed7fd5e12287f14e775715a4', 'p.rossi@reply.it', 'Paolo', 'Rossi', 'did:iota:test:dF7ET9vQGs1S5RDaVfCqcH6gc4coZ35mxJVZABrh2o2', 1),
    ('m222', '8a37ee8bd19d25b539e3b1b85cb3533ab6d8a691ed7fd5e12287f14e775715a4', 'f.nero@reply.it', 'Francesco', 'Neri', 'did:iota:test:7M3jibZDJXcVWFZwkewnVm2nRx3gJWkGLzJkY4Ho6o2K', 2);

INSERT INTO bioenpro4to.trucks (plate, did, driver) VALUES ('aa000aa', 'did:iota:test:2GxszDLXHWs4kJgwbz1ch5Gwh4MBBwZxJyaU3Yf6bHij', 'm111');

INSERT INTO bioenpro4to.scales (plant, did) VALUES ('CIDIU Druento', 'did:iota:test:H7PfugwjWzbexFES2t54tfEi2dmtQ9qEw6UZA8rPhmhg');

INSERT INTO bioenpro4to.biocells (id, plant, max_capacity) VALUES ('d111', 'did:iota:test:BG6DuW2ESTyvLR2CJA4GJAT53NfMJohZYjmfWRiGySeg', 2000);

