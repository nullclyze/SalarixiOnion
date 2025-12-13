import requests
import uvicorn
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel


app = FastAPI()

app.middleware(CORSMiddleware(
  app=app,
  allow_credentials=True,
  allow_methods=['*'],
  allow_origins=['*'],
  allow_headers=['*']
))


class Options(BaseModel):
  algorithm: str
  protocol: str
  country: str
  quantity: str


class ProxyCollector:
  def __init__(self):
    self.apic_services = {
      'proxyscrape': 'https://api.proxyscrape.com/v4/free-proxy-list/get?request=display_proxies&proxy_format=protocolipport&format=json',
      'proxifly': 'https://cdn.jsdelivr.net/gh/proxifly/free-proxy-list@main/proxies/all/data.json'
    }

    self.latest_list = None
    self.latest_options = None

  async def APIc(self, protocol: str, country: str):
    proxies = []

    for service in self.apic_services:
      response = requests.get(self.apic_services[service])
            
      data = response.json()
      
      if service == 'proxyscrape':
        proxy_list = data['proxies']

        for proxy in proxy_list:
          if country == 'any' or proxy['ip_data']['countryCode'] == country.upper():
            if protocol == 'any' or str(proxy['protocol']).startswith(protocol):
              proxies.append(proxy['proxy'])
      elif service == 'proxifly':
        for proxy in data:
          if country == 'any' or proxy['geolocation']['country'] == country.upper():
            if protocol == 'any' or str(proxy['protocol']).startswith(protocol):
              proxies.append(proxy['proxy'])

    return proxies
  
  async def collect(self, options: Options):
    try:
      dict_options = options.model_dump()

      if self.latest_options != None:
        if self.latest_options == dict_options:
          return { 'success': True, 'list': self.latest_list }
        
      self.latest_options = dict_options

      proxies = []

      if options.algorithm == 'apic':
        proxies = await self.APIc(options.protocol, options.country)

      if options.quantity != 'max':
        proxies = proxies[:int(options.quantity)]

      proxies = [str(p) for p in dict.fromkeys(proxies)]

      self.latest_list = proxies

      return { 'success': True, 'list': proxies }
    
    except Exception as err:
      return { 'success': False, 'error': err, 'list': None }

proxyCollector = ProxyCollector()

@app.post('/salarixi/web/collector/proxy')
async def collect(options: Options):
  return await proxyCollector.collect(options)

    
if __name__ == '__main__':
  uvicorn.run(app=app, host='localhost', port=37475)