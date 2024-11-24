from models.database import DataBase
import datetime

db=DataBase()

def get_projects():
    return db.request("SELECT * FROM project ORDER BY id DESC", ())

def create_project(name):
    date=datetime.datetime.now()
    return db.request("INSERT INTO project(name, creation_date) VALUES(%s, %s)", (name, date))

def get_project(project_id):
    project=db.request("SELECT * FROM project WHERE id=%s", (project_id))[0]
    if len(project>0):
        return project[0]
    return project

def get_websites(project_id):
    return db.request("SELECT * FROM website WHERE project_id=%s ORDER BY id DESC", (project_id))

def create_website(project_id, name):
    date=datetime.datetime.now()
    return db.request("INSERT INTO website(project_id, name, added_date) VALUES(%s, %s, %s)", (project_id, name, date))
