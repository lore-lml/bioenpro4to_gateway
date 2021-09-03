# BioEnPro4TO - Gateway

## Usage
- Clone the repository:
`git clone https://github.com/lore-lml/bioenpro4to_channel_manager.git`

- Modify the INSERTS statements in `db_docker_init/sql/init_db.sql` file as needed:
  - use `f91bad83ce31d38aa8fab39fbc3789b825bd67814f7b9994cdc3d062acfe6b34` in the password fields that is the hashed value of `ciao`
  - provides valid dids for each actor

- Modify the .env file as needed:
  - `PG.HOST=bep4t_db` must not be changed for a working docker build

- Run `docker compose up -d` in the root folder

