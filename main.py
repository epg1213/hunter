"""from models.crawler import Crawler

url="https://hackazon.trackflaw.com"
crawler = Crawler()
crawler.crawl(url)
print(crawler.visited)"""

from flask import Flask

app = Flask(__name__)

@app.route('/')
def index():
    return 'Web App with Python Flask!'

app.run(host='0.0.0.0', port=42601)
