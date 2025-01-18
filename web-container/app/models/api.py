from app.models import database
import datetime

db=database.DataBase()

def get_projects():
    return db.request("SELECT * FROM project ORDER BY id DESC", ())

def create_project(name):
    date=datetime.datetime.now()
    return db.request("INSERT INTO project(name, creation_date) VALUES(%s, %s)", (name, date))

def get_project(project_id):
    project=db.request("SELECT * FROM project WHERE id=%s", (project_id,))
    if len(project)>0:
        return project[0]
    return project

def get_websites(project_id):
    return db.request("SELECT * FROM website WHERE project_id=%s ORDER BY id DESC", (project_id,))

def create_website(project_id, name):
    date=datetime.datetime.now()
    return db.request("INSERT INTO website(project_id, name, added_date) VALUES(%s, %s, %s)", (project_id, name, date))

def get_website(website_id):
    website=db.request("SELECT * FROM website WHERE id=%s ORDER BY id DESC", (website_id,))
    if len(website)>0:
        return website[0]
    return website

def get_pages(website_id):
    return db.request("SELECT * FROM page WHERE website_id=%s ORDER BY id DESC", (website_id,))

def get_url(website_id):
    url=db.request("SELECT name FROM website WHERE id=%s ORDER BY id DESC", (website_id,))
    if len(url)>0:
        return url[0][0]
    return ''

def get_website_id(url):
    website_id=db.request("SELECT id FROM website WHERE name LIKE CONCAT(%s, '%') ORDER BY id DESC", (url,))
    if len(website_id)>0:
        return website_id[0][0]
    else:
        return 0
    
def get_website_data(website_id):
    data=db.request("SELECT * FROM page WHERE website_id=%s", (website_id,))
    res={}
    for page in data:
        res[page[0]]={
        "id": page[0],
        "website_id": page[1],
        "name": page[2],
        "max_content_length": page[3],
        "redirects": page[4]
        }
    return res

def save_response(baseURL, path, byte_count, redirect):
    website_id=get_website_id(baseURL)
    if website_id==0:
        return
    try:
        existing = db.request("SELECT max_content_length, id FROM page WHERE website_id=%s AND name=%s", (website_id, path))
        if len(existing)==0:
            db.request("INSERT INTO page(website_id, name, max_content_length, redirects) VALUES(%s, %s, %s, %s)",
            (website_id, path, byte_count, redirect))
        else:
            old_byte_count=existing[0][0]
            page_id=existing[0][1]
            if int(old_byte_count)<byte_count:
                db.request("UPDATE page set max_content_length=%s WHERE id=%s",(old_byte_count, page_id))
            if redirect!="":
                db.request("UPDATE page set redirects=%s WHERE id=%s",(redirect, page_id))
    except:
        pass
