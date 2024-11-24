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

app.run(host='0.0.0.0', port=42602)
