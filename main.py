"""from models.crawler import Crawler

url="https://hackazon.trackflaw.com"
crawler = Crawler()
crawler.crawl(url)
print(crawler.visited)"""

from flask import Flask, render_template, request
from models.api import *

app = Flask(__name__)

@app.route('/', methods=['GET', 'POST'])
def index():
    if request.method == 'POST' and 'name' in request.form:
        create_project(request.form['name'])
    return render_template('index.html', projects=get_projects())

@app.route('/project', methods=['GET', 'POST'])
def project():
    project_id=0
    if 'id' in request.args:
        project_id=request.args['id']
    if request.method == 'POST' and 'name' in request.form:
        create_website(project_id, request.form['name'])
    return render_template('project.html', project=get_project(project_id), websites=get_websites(project_id))

@app.route('/website')
def website():
    website_id=0
    if 'id' in request.args:
        website_id=request.args['id']
    return render_template('website.html', website=get_website(website_id), pages=get_pages(website_id))

app.run(host='0.0.0.0', port=42602)
