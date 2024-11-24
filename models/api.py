from models.database import DataBase
import datetime
db=DataBase()
def get_projects():
    return db.request("SELECT * FROM project", ())

def create_project(name):
    date=datetime.datetime.now()
    return db.request("INSERT INTO project(name, creation_date) VALUES(%s, %s)", (name, date))
