import { logger } from '../utils/logger';

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
  ],
  'json:proxyfreeonly': [
    'https://proxyfreeonly.com/api/free-proxy-list?limit=500&page=1&sortBy=lastChecked&sortType=desc',
    'https://proxyfreeonly.com/api/free-proxy-list?limit=500&page=2&sortBy=lastChecked&sortType=desc',
    'https://proxyfreeonly.com/api/free-proxy-list?limit=500&page=3&sortBy=lastChecked&sortType=desc',
  ],
};

const upcServices = {
  ...apicServices,
  'text:gfpcom-proxy-list': [
    'https://raw.githubusercontent.com/wiki/gfpcom/free-proxy-list/lists/http.txt',
    'https://raw.githubusercontent.com/wiki/gfpcom/free-proxy-list/lists/socks4.txt',
    'https://raw.githubusercontent.com/wiki/gfpcom/free-proxy-list/lists/socks5.txt'
  ],
  'text:fyvri-proxy-list': 'https://raw.githubusercontent.com/fyvri/fresh-proxy-list/archive/storage/classic/socks5.txt',
  'text:r00tee-proxy-list': 'https://raw.githubusercontent.com/r00tee/Proxy-List/main/Socks5.txt',
  'text:vmheaven-proxy-list': 'https://raw.githubusercontent.com/vmheaven/VMHeaven-Free-Proxy-Updated/refs/heads/main/socks5_anonymous.txt',
  'text:iplocate-proxy-list': 'https://raw.githubusercontent.com/iplocate/free-proxy-list/refs/heads/main/protocols/socks5.txt'
};

class ProxyCollector {
  private list: HTMLTextAreaElement | null;
  private counter: HTMLElement | null;
  private status: HTMLElement | null;

  constructor() {
    this.list = null;
    this.counter = null;
    this.status = null;
  }
  
  /** Метод инициализации функций, связанных с сборщиком прокси. */
  public init(): void {
    this.list = document.getElementById('proxy-list') as HTMLTextAreaElement;
    this.counter = document.getElementById('proxy-counter') as HTMLElement;
    this.status = document.getElementById('proxy-finder-status') as HTMLElement;

    this.list.addEventListener('input', () => this.updateCounter());

    document.getElementById('clear-proxy-list')?.addEventListener('click', () => {
      this.list!.value = '';
      this.updateCounter();
    });

    document.getElementById('find-proxy')?.addEventListener('click', () => this.collectProxy());

    this.updateCounter();
  }

  /** Метод обновления счётчика прокси. */
  private updateCounter(): void {
    let counter = 0;
    this.list!.value.split('\n').forEach(element => element.match(/((?:\d{1,3}\.){3}\d{1,3}):(\d+)/g) || element.match(/(\w+)\:\/\/((?:\d{1,3}\.){3}\d{1,3}):(\d+)/g) ? counter++ : null);
    this.counter!.innerText = counter.toString();
  }

  /** Метод установки статуса поиска прокси. */
  private setStatus(text: string, color?: string): void {
    if (!this.status) return;
    this.status!.style.color = color ?? '#848080';
    this.status!.innerText = text;
  }
  
  /** Метод сборки прокси с различных ресурсов. */
  private async collectProxy(): Promise<void> {
    try {
      const algorithm = (document.getElementById('proxy-finder_select_algorithm') as HTMLSelectElement).value;
      const protocol = (document.getElementById('proxy-finder_select_protocol') as HTMLSelectElement).value;
      const country = (document.getElementById('proxy-finder_select_country') as HTMLSelectElement).value;
      const port = (document.getElementById('proxy-finder_select_port') as HTMLSelectElement).value;
      const count = (document.getElementById('proxy-finder_select_count') as HTMLInputElement).value;

      this.setStatus('Поиск прокси...');

      const proxies = await this.scrape({
        algorithm: algorithm,
        protocol: protocol,
        country: country,
        port: port,
        count: count
      });

      if (proxies) {
        this.setStatus('Поиск окончен', '#0cd212ff');
        this.list!.value = Array.from(String(proxies).split('\n')).filter(p => p && p.trim() !== '').join('\n');
      } else {
        this.setStatus('Ошибка поиска', '#cc1d1dff');
        logger.log('Ошибка сборщика прокси: Could not find a proxies', 'error');
      }
    } catch (error) {
      this.setStatus('Ошибка поиска', '#cc1d1dff');
      logger.log(`Ошибка сборщика прокси: ${error}`, 'error');
    } finally {
      this.updateCounter();
      setTimeout(() => this.setStatus('Поиск неактивен'), 2000);
    }
  }

  /** Метод скрапинга прокси. */
  private async scrape(options: Options): Promise<string | null> {
    try {
      let proxy_list: any[] = [];

      if (options.algorithm === 'apic') {
        proxy_list = await this.APIc(options.protocol, options.country);
      } else if (options.algorithm === 'upc') {
        proxy_list = await this.UPc(options.protocol);
      }

      if (options.count !== 'max') proxy_list = proxy_list.slice(0, Number(options.count));

      proxy_list = Array.from(new Set(proxy_list.map(String)));

      let result: string[] = [];

      if (options.port != 'any') {
        for (const proxy of proxy_list) proxy.includes(options.port) ? result.push(proxy) : null;
      } else {
        for (const proxy of proxy_list) result.push(proxy);
      }

      return result.join('\n');
    } catch (_) {
      return null;
    }
  }

  /** Метод отправки запроса на определённый URL. */
  private async request(name: string, url: string, type: string) {
    try {
      const res = await fetch(url, {
        signal: AbortSignal.timeout(6000)
      });

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

  /** Метод создания потока задач (потока запросов). */
  private createTaskFlow(urls: any) {
    const tasks = [];

    for (const [name, url] of Object.entries(urls)) {
      if (Array.isArray(url)) {
        for (const u of url) tasks.push(this.request(name.split(':')[1], u, name.split(':')[0]));
      } else {
        tasks.push(this.request(name.split(':')[1], url as string, name.split(':')[0]));
      }
    }

    return tasks;
  }

  /** Метод поиска прокси по алгоритму UPc. */
  private async UPc(protocol: string): Promise<any[]> {
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
        for (const proxy of data ?? []) protocol === 'any' || proxy.split('://')[0].toLowerCase() === protocol ? proxies.push(proxy) : null;
      } else if (name.includes('fyvri-proxy-list') || name.includes('r00tee-proxy-list') || name.includes('vmheaven-proxy-list') || name.includes('iplocate-proxy-list')) {
        for (const proxy of data ?? []) protocol === 'socks5' ? proxies.push(`socks5://${proxy}`) : null;
      }
    } 

    return proxies;
  }

  /** Метод поиска прокси по алгоритму APIc. */
  private async APIc(protocol: string, country: string): Promise<any[]> {
    const tasks = this.createTaskFlow(apicServices);
    const results = await Promise.all(tasks);

    const proxies = [];

    for (const r of results) {
      if (!r.success) continue;

      const data = r.data;
      const name = r.name;

      if (name === 'proxyscrape') {
        for (const proxy of data.proxies ?? []) country === 'any' || proxy.ip_data?.countryCode === country.toUpperCase() ? protocol === 'any' || proxy.protocol.toLowerCase() === protocol ? proxies.push(proxy.proxy) : null : null;
      } else if (name === 'proxifly') {
        for (const proxy of data ?? []) country === 'any' || proxy.geolocation?.country === country.toUpperCase() ? protocol === 'any' || proxy.protocol.toLowerCase() === protocol ? proxies.push(proxy.proxy) : null : null;
      } else if (name === 'jetkai-proxy-list') {
        for (const proxy of data ?? []) {
          if (country === 'any' || proxy.location?.isocode === country.toUpperCase()) {
            const type = String(proxy.protocols?.[0]?.type);
            protocol === 'any' || type.toLowerCase() === protocol ? proxies.push(`${type}://${proxy.ip}:${proxy.port}`) : null;
          }
        }
      } else if (name === 'monosans-proxy-list') {
        for (const proxy of data ?? []) proxy.username == null && proxy.password == null ? country === 'any' || proxy.geolocation?.country?.iso_code === country.toUpperCase() ? protocol === 'any' || String(proxy.protocol).toLowerCase() === protocol ? proxies.push(`${proxy.protocol}://${proxy.host}:${proxy.port}`) : null : null : null;
      } else if (name === 'vakhov-proxy-list') {
        for (const proxy of data ?? []) {
          if (country === 'any' || proxy.country_code === country.toUpperCase()) {
            if (protocol === 'any') {
              for (const p of ['socks5', 'socks4', 'http']) Number(proxy[p]) !== 0 ? proxies.push(`${p}://${proxy.ip}:${proxy.port}`) : null;
            } else {
              Number(proxy[protocol]) !== 0 ? proxies.push(`${protocol}://${proxy.ip}:${proxy.port}`) : null;
            }
          }
        }
      } else if (name === 'geonode' || name === 'proxyfreeonly') {
        for (const proxy of data ?? []) if (country === 'any' || proxy.country === country.toUpperCase()) for (const p of (proxy.protocols ?? []).map((p: string) => p.toLowerCase())) protocol === 'any' || p === protocol ? proxies.push(`${p}://${proxy.ip}:${proxy.port}`) : null;
      }
    }

    return proxies;
  }
}        

const proxyCollector = new ProxyCollector();

export { proxyCollector }