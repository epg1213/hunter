import requests
from html.parser import HTMLParser

class LinksInHTMLParser(HTMLParser):
  def get_values_from_attrs(self, attrs, key):
    values=[]
    for attr in attrs:
      if attr[0]==key:
        values.append(attr[1])
    return values
  
  def handle_starttag(self, tag, attrs):
    if tag=='a':
      self.links+=self.get_values_from_attrs(attrs, 'href')
    elif tag=='form':
      self.links+=self.get_values_from_attrs(attrs, 'action')
  
  def parse_html_content(self, content):
    self.links=[]
    self.feed(content)
    return self.links

class Crawler:
  def crawl(self, url, depth=5):
    protocol, _, domain=url.split('/')
    fetched=self.get_links(url)['internal']
    links=fetched['pages']
    allurls=fetched['raw']
    visited=[url]
    i=1
    while i<depth:
      i+=1
      new_links=[]
      for link in links:
        new_url=f"{protocol}//{domain}/{link}"
        if new_url in visited:
          continue
        fetched=self.get_links(new_url)['internal']
        new_links+=fetched['pages']
        allurls+=fetched['raw']
        visited.append(new_url)
      links=list(set(links+new_links))
      allurls=list(set(allurls))
    result={}
    for page in links:
      result[page]=[]
      for url in allurls:
        if url.split('?')[0].split('#')[0]==page:
          result[page].append(url)
    return result

  def get_links(self, url):
    response = requests.get(url, allow_redirects=False)
    parser = LinksInHTMLParser()
    links=parser.parse_html_content(response.text)

    external=[]
    external_raw=[]
    internal=[]
    internal_raw=[]
    for link in links:
      page=link.split('?')[0].split('#')[0]
      if link.startswith('https://') or link.startswith('http://'):
        external.append(page)
        external_raw.append(link)
      elif link.startswith('/'):
        internal.append(page[1:])
        internal_raw.append(link[1:])
      elif not link.startswith('mailto:'):
        internal.append(page)
        internal_raw.append(link)
    return {
      "external":{
        "pages":list(set(external)),
        "raw":list(set(external_raw))
      },
      "internal":{
        "pages":list(set(internal)),
        "raw":list(set(internal_raw))
      }
    }
