import requests
from models.api import save_response

class RequestMaker():
  def __init__(self):
    self.headers={'User-agent': "https://github.com/epg1213/hunter"}

  def set_cookie(self, cookie):
    self.headers['Cookie'] = cookie

  def make_request(self, baseURL, link):
    method, netloc, path, params = link
    if netloc!='':
      return 0
    if path.startswith('/'):
      url=f"{baseURL}{path}"
    else:
      url=f"{baseURL}/{path}"
    match method:
      case 'GET':
        response = requests.get(url, params, allow_redirects=False, headers=self.headers)
      case 'POST':
        response = requests.post(url, params, allow_redirects=False, headers=self.headers)
      case 'PUT':
        response = requests.put(url, params, allow_redirects=False, headers=self.headers)
      case 'DELETE':
        response = requests.delete(url, allow_redirects=False, headers=self.headers)
    if "location" in response.headers and response.status_code<400 and response.status_code>299:
      redirect=response.headers["location"]
    else:
      redirect=''
    byte_count=len(response.text)
    save_response(baseURL, path, byte_count, redirect)
    return response
