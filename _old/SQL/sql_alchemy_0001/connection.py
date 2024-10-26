import psycopg
from sqlalchemy import create_engine

engine = create_engine((
    "postgresql+psycopg://"
    "postgres:"
    "1111@"
    "0.0.0.0:"
    "5432/"
    "rnddb"
))
