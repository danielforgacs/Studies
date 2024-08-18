import connection
import sqlalchemy.orm as orm
from sqlalchemy.orm import DeclarativeBase

class Base(DeclarativeBase):
    pass


class Shit(Base):
    __tablename__ = 'shit'

    id: orm.Mapped[int] = orm.mapped_column(primary_key=True)


def migrate():
    Base.metadata.create_all(connection.engine)


if __name__ == '__main__':
    migrate()
