import connection
import sqlalchemy.orm as orm
from sqlalchemy.orm import DeclarativeBase
from sqlalchemy import String

class Base(DeclarativeBase):
    pass


class Shit(Base):
    __tablename__ = 'shit'

    id: orm.Mapped[int] = orm.mapped_column(primary_key=True)
    name: orm.Mapped[str] = orm.mapped_column(String(30))


def migrate():
    print('::migrating')
    Base.metadata.create_all(connection.engine)


if __name__ == '__main__':
    migrate()
