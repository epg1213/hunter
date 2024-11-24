from models.database import DataBase
db=DataBase()
def get_projects():
    return db.request("SELECT * FROM project", ())

