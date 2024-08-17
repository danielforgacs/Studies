(1) - create docker network (or won't be able to connect to the docker process)

    $ docker network create pgnet

(2) - start docker postgres image

    $ docker run -e POSTGRES_PASSWORD=1111 --network pgnet -p 5432:5432 postgres

(3) - run psql in docker postgres

    $ docker exec -ti <CONTAINER ID>  psql -U postgres