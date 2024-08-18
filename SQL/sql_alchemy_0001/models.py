from sqlalchemy.schema import MetaData
from sqlalchemy.orm import DeclarativeBase


class Base(DeclarativeBase):
    pass


def migrate():
    MetaData.create_all()
