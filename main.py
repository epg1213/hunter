"""from models.crawler import Crawler

url="https://hackazon.trackflaw.com"
crawler = Crawler()
crawler.crawl(url)
print(crawler.visited)"""

from flask import Flask, render_template, request
import logging
from models.api import *

app = Flask(__name__)
log = logging.getLogger('werkzeug')
log.setLevel(logging.ERROR)

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

app.run(host='0.0.0.0', port=42602)
