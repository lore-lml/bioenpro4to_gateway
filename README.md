# BioEnPro4TO - Gateway

## Usage
Clone the repository:
`git clone https://github.com/lore-lml/bioenpro4to_channel_manager.git`
Then execute these instructions:
- `cd db_docker_init`
- `docker build -t bioenpro4to/postgres:0.1 .`
- `docker run -d -p 5432:5432 --name postgres-bioenpro4to bioenpro4to/postgres:0.1`
- `cd ..`
- `cargo run`

