import requests
from html.parser import HTMLParser
from urllib.parse import urlparse
from urllib.parse import parse_qs

class LinksInHTMLParser(HTMLParser):
  def get_values_from_attrs(self, attrs, key):
    values=[]
    for attr in attrs:
      if attr[0]==key:
        values.append(attr[1])
    return values

  def handle_tag_a(self, attrs):
    try:
      parsed_url = urlparse(self.get_values_from_attrs(attrs, 'href')[0])
    except IndexError:
      return None
    if not parsed_url.scheme in ['', 'http', 'https']:
      return None
    method='GET'
    netloc=parsed_url.netloc
    page=parsed_url.path
    params=parse_qs(parsed_url.query)
    return (method, netloc, page, params)

  def handle_tag_form(self, attrs):
    self.in_form=True
    try:
      parsed_url = urlparse(self.get_values_from_attrs(attrs, 'action')[0])
      netloc=parsed_url.netloc
      page=parsed_url.path
    except IndexError:
      netloc=''
      page=''
    try:
      method = self.get_values_from_attrs(attrs, 'method')[0].upper()
    except IndexError:
      method = 'GET'
    self.form_data=[method, netloc, page, {}]

  def handle_tag_input_textarea(self, attrs):
    try:
      name = self.get_values_from_attrs(attrs, 'name')[0]
      try:
        value = self.get_values_from_attrs(attrs, 'value')[0]
      except IndexError:
        value=''
      if name in self.form_data[3]:
        self.form_data[3][name].append(value)
      else:
        self.form_data[3][name]=[value]
    except IndexError:
      pass

  def handle_starttag(self, tag, attrs):
    if tag=='a':
      link=self.handle_tag_a(attrs)
      if link is not None and not link in self.links:
        self.links.append(link)
    elif tag=='form':
      self.handle_tag_form(attrs)
    elif tag=='input' or tag=='textarea':
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
  def make_request(self, link):
    method, netloc, path, params = link
    if netloc!='':
      return 0
    if path.startswith('/'):
      url=f"{self.baseURL}{path}"
    else:
      url=f"{self.baseURL}/{path}"
    match method:
      case 'GET':
        response = requests.get(url, params, allow_redirects=False)
      case 'POST':
        response = requests.post(url, params, allow_redirects=False)
      case 'PUT':
        response = requests.put(url, params, allow_redirects=False)
      case 'DELETE':
        response = requests.delete(url, allow_redirects=False)
    self.visited[link]=(response.code, response.headers)
    return response.text

  def crawl(self, url, depth=5):
    parsed_url = urlparse(url)
    self.scheme=parsed_url.scheme if parsed_url.scheme in ['https', 'http'] else 'https'
    self.netloc=parsed_url.netloc
    self.baseURL = f"{self.scheme}://{self.netloc}"
    self.visited={}

    links=self.get_links(('GET', '', parsed_url.path, {}))
    i=1
    while i<depth:
      i+=1
      new_links=[]
      for link in links:
        if link in self.visited:
          continue
        fetched=self.get_links(link)
        new_links+=fetched
      links=list(set(links+new_links))

  def get_links(self, link):
    response = self.make_request(link)
    parser = LinksInHTMLParser()
    return parser.parse_html_content(response.text)
