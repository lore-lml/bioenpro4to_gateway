# Build and run:
#   docker build -t bioenpro4to/postgres:0.1 .
#   docker run -d -p 5432:5432 --name postgres-bioenpro4to bioenpro4to/postgres:0.1
FROM postgres:13-alpine
ENV POSTGRES_USER admin
ENV POSTGRES_PASSWORD bioenpro4to
ENV POSTGRES_DB bioenpro4to
ADD ./sql/init_db.sql /docker-entrypoint-initdb.d
EXPOSE 5432
