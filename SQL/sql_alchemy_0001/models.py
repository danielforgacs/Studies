import connection
from sqlalchemy.orm import DeclarativeBase


class Base(DeclarativeBase):
    pass


def migrate():
    Base.metadata.create_all(connection.engine)


if __name__ == '__main__':
    migrate()
