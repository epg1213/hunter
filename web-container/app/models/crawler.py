from html.parser import HTMLParser
from urllib.parse import urlparse
from urllib.parse import parse_qs
from app.models.browser import RequestMaker

class LinksInHTMLParser(HTMLParser):
  def get_value_from_attrs(self, attrs, key):
    for attr in attrs:
      if attr[0]==key:
        return attr[1]
    return ''

  def handle_tag_a(self, attrs):
    parsed_url = urlparse(self.get_value_from_attrs(attrs, 'href'))
    if not parsed_url.scheme in ['', 'http', 'https']:
      return
    method='GET'
    netloc=parsed_url.netloc
    page=parsed_url.path
    params={}
    for key, value in parse_qs(parsed_url.query).items():
      params[key]=value[0]
    link=(method, netloc, page, params)
    if not link in self.links:
      self.links.append(link)

  def handle_tag_form(self, attrs):
    self.in_form=True
    method = self.get_value_from_attrs(attrs, 'method').upper()
    if method=='':
      method = 'GET'
    parsed_url = urlparse(self.get_value_from_attrs(attrs, 'action'))
    netloc=parsed_url.netloc
    page=parsed_url.path
    self.form_data=[method, netloc, page, {}]

  def handle_tag_input_textarea(self, attrs):
    name = self.get_value_from_attrs(attrs, 'name')
    value = self.get_value_from_attrs(attrs, 'value')
    self.form_data[3][name]=value

  def handle_starttag(self, tag, attrs):
    match tag:
      case 'a':
        self.handle_tag_a(attrs)
      case 'form':
        self.handle_tag_form(attrs)
      case 'input' | 'textarea':
        self.handle_tag_input_textarea(attrs)

  def handle_endtag(self, tag):
    if tag=='form':
      self.in_form=False
      if len(self.form_data)>0 and not tuple(self.form_data) in self.links:
        self.links.append(tuple(self.form_data))

  def parse_html_content(self, content):
    self.links=[]
    self.in_form=False
    self.form_data=[]
    self.feed(content)
    return self.links

class Crawler:
  def __init__(self, depth=5, max_visits=10):
    self.browser=RequestMaker()
    self.depth=depth
    self.max_visits=max_visits

  def crawl(self, url):
    parsed_url = urlparse(url)
    self.scheme=parsed_url.scheme if parsed_url.scheme in ['https', 'http'] else 'https'
    self.netloc=parsed_url.netloc
    self.baseURL = f"{self.scheme}://{self.netloc}"
    self.visited={}

    links=self.get_links(('GET', '', parsed_url.path, {}))
    i=1
    while i<self.depth:
      i+=1
      new_links=[]
      for link in links:
        visit_id=f"{link[0]}:{link[2]}"
        if visit_id in self.visited and self.visited[visit_id]>=self.max_visits:
          continue
        fetched=self.get_links(link)
        new_links+=fetched
      links=new_links

  def get_links(self, link):
    response = self.browser.make_request(self.baseURL, link)
    if 'Set-Cookie' in response.headers:
      self.browser.set_cookie(response.headers['Set-Cookie'])
    visit_id=f"{link[0]}:{link[2]}"
    if visit_id in self.visited:
      self.visited[f"{link[0]}:{link[2]}"]+=1
    else:
      self.visited[f"{link[0]}:{link[2]}"]=1
    return LinksInHTMLParser().parse_html_content(response.text)
