import os

from piccolo.conf.apps import AppRegistry
from piccolo.engine.sqlite import SQLiteEngine

# Connect to your Sqlite database
DB_PATH: str = os.path.join(os.path.dirname(__file__), "my_app.sqlite")

DB = SQLiteEngine(path=DB_PATH)

# Register your app (assuming your models are in main.py)
APP_REGISTRY = AppRegistry(apps=["robyn_app"])
