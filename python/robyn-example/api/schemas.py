from piccolo.columns import Integer, Serial, Varchar
from piccolo.table import Table


class User(Table, tablename="users"):
    id = Serial(primary_key=True)
    username = Varchar(length=50, unique=True, null=False)
    age = Integer(null=False)
