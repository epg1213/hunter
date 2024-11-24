"""from models.crawler import Crawler

url="https://hackazon.trackflaw.com"
crawler = Crawler()
crawler.crawl(url)
print(crawler.visited)"""

from flask import Flask, render_template
import logging

app = Flask(__name__)
log = logging.getLogger('werkzeug')
log.setLevel(logging.ERROR)

@app.route('/')
def index():
    return render_template('index.html', projects=[[1, "superprojet", ""], [2, "testprojet", ""]])

app.run(host='0.0.0.0', port=42602)
