import models

from sqlalchemy.orm import Session

spongebob = models.User(
    name="spongebob",
    fullname="Spongebob Squarepants",
    addresses=[models.Address(email_address="spongebob@sqlalchemy.org")],
)
sandy = models.User(
    name="sandy",
    fullname="Sandy Cheeks",
    addresses=[
        models.Address(email_address="sandy@sqlalchemy.org"),
        models.Address(email_address="sandy@squirrelpower.org"),
    ],
)
patrick = models.User(name="patrick", fullname="Patrick Star")

with Session(models.engine) as session:
    session.add_all([spongebob, sandy, patrick])
    session.commit()
