import requests
from html.parser import HTMLParser

def crawl(url, depth=5):
  protocol=url.split(':')[0]
  domain=url.split('/')[2]
  fetched=get_links(url)['internal']
  links=fetched['pages']
  allurls=fetched['raw']
  visited=[url]
  i=1
  while i<depth:
    i+=1
    new_links=[]
    for link in links:
      new_url=f"{protocol}://{domain}{link}"
      if new_url in visited:
        continue
      fetched=get_links(new_url)['internal']
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

def get_values_from_attrs(attrs, key):
  values=[]
  for attr in attrs:
    if attr[0]==key:
      values.append(attr[1])
  return values

def get_links(url):
  class LinksInHTMLParser(HTMLParser):
    def handle_starttag(self, tag, attrs):
      if not hasattr(self, 'links'):
        self.links=[]
      if tag=='a':
        self.links+=get_values_from_attrs(attrs, 'href')
      elif tag=='form':
        self.links+=get_values_from_attrs(attrs, 'action')

  links=[]
  parser = LinksInHTMLParser()
  response = requests.get(url, allow_redirects=False)
  parser.feed(response.text)
  if not hasattr(parser, 'links'):
    return {"external":{"pages":[], "raw":[]}, "internal":{"pages":[], "raw":[]}}
  external=[]
  external_raw=[]
  internal=[]
  internal_raw=[]
  for link in parser.links:
    page=link.split('?')[0].split('#')[0]
    if link.startswith('https://') or link.startswith('http://'):
      external.append(page)
      external_raw.append(link)
    elif link.startswith('/'):
      internal.append(page)
      internal_raw.append(link)
    elif not link.startswith('mailto:'):
      internal.append(f"/{page}")
      internal_raw.append(f"/{link}")
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

#url=input('url > ')
url="https://hackazon.trackflaw.com"
print(crawl(url))
#print(scan(url))
