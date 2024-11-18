import requests

class RequestMaker():
  self.headers=={'User-agent': "https://github.com/epg1213/hunter"}

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
    return response
