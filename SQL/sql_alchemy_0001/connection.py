import psycopg
from sqlalchemy import create_engine

engine = create_engine((
    "postgresql+psycopg://"
    "postgres:"
    "pass@"
    "localhost:"
    "5432/"
    "rnddb"
))
