DROP SCHEMA IF EXISTS bioenpro4to CASCADE;
CREATE SCHEMA bioenpro4to;

CREATE TABLE bioenpro4to.users (
    id VARCHAR(200) NOT NULL,
    email VARCHAR(200) NOT NULL,
    first_name VARCHAR(200) NOT NULL,
    last_name VARCHAR(200) NOT NULL,
    did VARCHAR(200),
    PRIMARY KEY(id)
);

INSERT INTO bioenpro4to.users (id, email, first_name, last_name)
VALUES
    ('m111', 'p.rossi@reply.it', 'Paolo', 'Rossi'),
    ('m222', 'f.neri@reply.it', 'Francesco', 'Neri'),
    ('m333', 'g.bianchi@reply.it', 'Giuseppe', 'Bianchi');
