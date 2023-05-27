# in the book: scripts/wait_for_database.sh
# cd ..

docker-compose up -d

# pg_ready is not installed:
# until pg_isready -h localhost -p 5432 -U username

# alternative solution for pg_ready:
until docker run -it postgres --add-host host.docker.internal:host-gateway docker.io/postgres:14-alpine -h localhost -U username pg_isready
do
  echo "Waiting for postgres"
  sleep 2;
done
echo "docker is now running"
docker-compose down