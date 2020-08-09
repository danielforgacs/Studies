FROM ubuntu
# FROM ubuntu as builder

WORKDIR /home

RUN apt-get update
RUN apt-get install -y --no-install-recommends pipenv
RUN pipenv install --deploy django==3.0.8

# FROM ubuntu

# WORKDIR /home

# COPY --from=builder /home/Pipenv .
# COPY --from=builder /home/Pipenv.lock .
