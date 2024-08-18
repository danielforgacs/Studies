import psycopg
from sqlalchemy import create_engine

engine = create_engine((
    "postgresql+psycopg://"
    "scott:"
    "tiger@"
    "localhost:"
    "5432/"
    "mydatabase"
    ))
