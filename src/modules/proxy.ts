import { log } from '../logger';


interface Options { 
  algorithm: string; 
  protocol: string; 
  country: string; 
  port: string;
  count: string;
};

const apicServices = {
  'json:proxyscrape': 'https://api.proxyscrape.com/v4/free-proxy-list/get?request=display_proxies&proxy_format=protocolipport&format=json',
  'json:proxifly': 'https://cdn.jsdelivr.net/gh/proxifly/free-proxy-list@main/proxies/all/data.json',
  'json:jetkai-proxy-list': 'https://raw.githubusercontent.com/jetkai/proxy-list/main/online-proxies/json/proxies-advanced.json',
  'json:monosans-proxy-list': 'https://raw.githubusercontent.com/monosans/proxy-list/main/proxies_pretty.json',
  'json:vakhov-proxy-list': 'https://raw.githubusercontent.com/vakhov/fresh-proxy-list/refs/heads/master/proxylist.json',
  'json:geonode': [
    'https://proxylist.geonode.com/api/proxy-list?limit=500&page=1&sort_by=lastChecked&sort_type=desc',
    'https://proxylist.geonode.com/api/proxy-list?limit=500&page=2&sort_by=lastChecked&sort_type=desc',
    'https://proxylist.geonode.com/api/proxy-list?limit=500&page=3&sort_by=lastChecked&sort_type=desc',
  ]
};

const upcServices = {
  ...apicServices,
  'text:gfpcom-proxy-list': [
    'https://raw.githubusercontent.com/wiki/gfpcom/free-proxy-list/lists/http.txt',
    'https://raw.githubusercontent.com/wiki/gfpcom/free-proxy-list/lists/socks4.txt',
    'https://raw.githubusercontent.com/wiki/gfpcom/free-proxy-list/lists/socks5.txt',
    'https://raw.githubusercontent.com/wiki/gfpcom/free-proxy-list/lists/ss.txt'
  ],
  'text:fyvri-proxy-list': 'https://raw.githubusercontent.com/fyvri/fresh-proxy-list/archive/storage/classic/socks5.txt',
  'text:r00tee-proxy-list': 'https://raw.githubusercontent.com/r00tee/Proxy-List/main/Socks5.txt',
  'text:vmheaven-proxy-list': 'https://raw.githubusercontent.com/vmheaven/VMHeaven-Free-Proxy-Updated/refs/heads/main/socks5_anonymous.txt',
  'text:iplocate-proxy-list': 'https://raw.githubusercontent.com/iplocate/free-proxy-list/refs/heads/main/protocols/socks5.txt'
};

export class ProxyManager {
  private proxyList: HTMLTextAreaElement | null = null;
  private proxyCounter: HTMLElement | null = null;

  private proxyFinderStatus: HTMLElement | null = null;

  public init(): void {
    this.proxyList = document.getElementById('proxy-list') as HTMLTextAreaElement;
    this.proxyFinderStatus = document.getElementById('proxy-finder-status') as HTMLElement;
    this.proxyCounter = document.getElementById('proxy-counter') as HTMLElement;

    this.proxyList.addEventListener('input', () => this.updateCount());

    document.getElementById('clear-proxy-list')?.addEventListener('click', () => {
      this.proxyList!.value = '';
      this.updateCount();
    });

    document.getElementById('find-proxy')?.addEventListener('click', () => this.collectProxy());

    this.updateCount();
  }

  private updateCount(): void {
    let counter = 0;

    String(this.proxyList!.value).split('\n').forEach(element => {
      if (element.match(/((?:\d{1,3}\.){3}\d{1,3}):(\d+)/g) || element.match(/(\w+)\:\/\/((?:\d{1,3}\.){3}\d{1,3}):(\d+)/g)) {
        counter++;
      }
    });

    this.proxyCounter!.innerText = counter.toString();
  }
  
  private async collectProxy(): Promise<void> {
    try {
      const algorithm = (document.getElementById('proxy-finder-algorithm') as HTMLSelectElement).value;
      const protocol = (document.getElementById('proxy-finder-protocol') as HTMLSelectElement).value;
      const country = (document.getElementById('proxy-finder-country') as HTMLSelectElement).value;
      const port = (document.getElementById('proxy-finder-port') as HTMLSelectElement).value;
      const count = (document.getElementById('proxy-finder-count') as HTMLInputElement).value;

      this.proxyFinderStatus!.innerText = 'Поиск прокси...';

      const proxies = await this.scrape({
        algorithm: algorithm,
        protocol: protocol,
        country: country,
        port: port,
        count: count
      });

      if (proxies) {
        this.proxyFinderStatus!.style.color = '#0cd212ff';
        this.proxyFinderStatus!.innerText = 'Поиск окончен';
        this.proxyList!.value = Array.from(String(proxies).split('\n')).filter(p => p && p.trim() !== '').join('\n');
      } else {
        this.proxyFinderStatus!.style.color = '#cc1d1dff';
        this.proxyFinderStatus!.innerText = 'Ошибка поиска';

        log(`Ошибка поиска прокси`, 'error');
      }
    } catch (error) {
      this.proxyFinderStatus!.style.color = '#cc1d1dff';
      this.proxyFinderStatus!.innerText = 'Ошибка поиска';

      log(`Ошибка поиска прокси: ${error}`, 'error');
    } finally {
      this.updateCount();
      setTimeout(() => {
        this.proxyFinderStatus!.style.color = '#848080';
        this.proxyFinderStatus!.innerText = 'Поиск неактивен';
      }, 2000);
    }
  }

  private async scrape(options: Options): Promise<string | unknown> {
    try {
      let proxy_list: any[] = [];

      if (options.algorithm === 'apic') {
        proxy_list = await this.APIc(options.protocol, options.country);
      } else if (options.algorithm === 'upc') {
        proxy_list = await this.UPc(options.protocol);
      }

      if (options.count !== 'max') {
        proxy_list = proxy_list.slice(0, Number(options.count));
      }

      proxy_list = Array.from(new Set(proxy_list.map(String)));

      let result: string[] = [];

      if (options.port != 'any') {
        for (const proxy of proxy_list) {
          if (proxy.includes(options.port)) {
            result.push(proxy);
          }
        }
      } else {
        for (const proxy of proxy_list) {
          result.push(proxy);
        }
      }

      return result.join('\n');
    } catch (error) {
      return error;
    }
  }

  private async request(name: string, url: string, type: string) {
    try {
      const res = await fetch(url);

      if (res.ok) {
        const data = type === 'json' ? await res.json() : await res.text();
        return { success: true, name: name, data: data };
      } else {
        return { success: false, name: name, error: `HTTP ${res.status}` };
      }
    } catch (error) {
      return { success: false, name: name, error: String(error) };
    }
  }

  private createTaskFlow(urls: any) {
    const tasks = [];

    for (const [name, url] of Object.entries(urls)) {
      if (Array.isArray(url)) {
        for (const u of url) tasks.push(this.request(String(name.split(':')[1]), u, String(name.split(':')[0])));
      } else {
        tasks.push(this.request(String(name.split(':')[1]), url as string, String(name.split(':')[0])));
      }
    }

    return tasks;
  }

  private async UPc(protocol: string) {
    let proxies = [];

    const apicList = await this.APIc(protocol, 'any');

    if (apicList.length > 0) proxies.push(...apicList);

    const tasks = this.createTaskFlow(upcServices);
    const results = await Promise.all(tasks);

    for (const r of results) {
      if (!r.success) continue;
      const data = String(r.data).trim().split('\n');
      const name = r.name;

      if (name.includes('gfpcom-proxy-list')) {
        for (const proxy of data ?? []) {
          if (protocol === 'any' || String(proxy.split('://')[0]).toLowerCase() === protocol) proxies.push(proxy);
        }
      } else if (name.includes('fyvri-proxy-list') || name.includes('r00tee-proxy-list') || name.includes('vmheaven-proxy-list') || name.includes('iplocate-proxy-list')) {
        for (const proxy of data ?? []) {
          if (protocol === 'socks5') proxies.push(`socks5://${proxy}`);
        }
      }
    } 

    return proxies;
  }

  private async APIc(protocol: string, country: string) {
    const tasks = this.createTaskFlow(apicServices);
    const results = await Promise.all(tasks);

    const proxies = [];

    for (const r of results) {
      if (!r.success) continue;
      const data = r.data;
      const name = r.name;

      if (name === 'proxyscrape') {
        for (const proxy of data.proxies ?? []) {
          if (country === 'any' || proxy.ip_data?.countryCode === country.toUpperCase()) {
            if (protocol === 'any' || String(proxy.protocol).toLowerCase() === protocol) proxies.push(proxy.proxy);
          }
        }
      } else if (name === 'proxifly') {
        for (const proxy of data ?? []) {
          if (country === 'any' || proxy.geolocation?.country === country.toUpperCase()) {
            if (protocol === 'any' || String(proxy.protocol).toLowerCase() === protocol) proxies.push(proxy.proxy);
          }
        }
      } else if (name === 'jetkai-proxy-list') {
        for (const proxy of data ?? []) {
          if (country === 'any' || proxy.location?.isocode === country.toUpperCase()) {
            const type = proxy.protocols?.[0]?.type;
            if (protocol === 'any' || String(type).toLowerCase() === protocol) {
              proxies.push(`${type}://${proxy.ip}:${proxy.port}`);
            }
          }
        }
      } else if (name === 'monosans-proxy-list') {
        for (const proxy of data ?? []) {
          if (proxy.username == null && proxy.password == null) {
            if (country === 'any' || proxy.geolocation?.country?.iso_code === country.toUpperCase()) {
              if (protocol === 'any' || String(proxy.protocol).toLowerCase() === protocol) {
                proxies.push(`${proxy.protocol}://${proxy.host}:${proxy.port}`);
              }
            }
          }
        }
      } else if (name === 'vakhov-proxy-list') {
        for (const proxy of data ?? []) {
          if (country === 'any' || proxy.country_code === country.toUpperCase()) {
            if (protocol === 'any') {
              for (const p of ['socks5', 'socks4', 'http']) {
                if (Number(proxy[p]) !== 0) proxies.push(`${p}://${proxy.ip}:${proxy.port}`);
              }
            } else {
              if (Number(proxy[protocol]) !== 0) proxies.push(`${protocol}://${proxy.ip}:${proxy.port}`);
            }
          }
        }
      } else if (name === 'geonode') {
        for (const proxy of data.data ?? []) {
          if (country === 'any' || proxy.country === country.toUpperCase()) {
            const protocols = (proxy.protocols ?? []).map((p: string) => p.toLowerCase());
            if (protocol === 'any') {
              for (const p of protocols) proxies.push(`${p}://${proxy.ip}:${proxy.port}`);
            } else {
              for (const p of protocols) if (p === protocol) proxies.push(`${p}://${proxy.ip}:${proxy.port}`);
            }
          }
        }
      }
    }

    return proxies;
  }
}        