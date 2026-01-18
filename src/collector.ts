interface Options { 
  algorithm: string; 
  protocol: string; 
  country: string; 
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

async function request(name: string, url: string, type: string) {
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

function createTaskFlow(urls: any) {
  const tasks = [];

  for (const [name, url] of Object.entries(urls)) {
    if (Array.isArray(url)) {
      for (const u of url) tasks.push(request(String(name.split(':')[1]), u, String(name.split(':')[0])));
    } else {
      tasks.push(request(String(name.split(':')[1]), url as string, String(name.split(':')[0])));
    }
  }

  return tasks;
}

async function UPc(protocol: string) {
  let proxies = [];

  const apicList = await APIc(protocol, 'any');

  if (apicList.length > 0) proxies.push(...apicList);

  const tasks = createTaskFlow(upcServices);
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

async function APIc(protocol: string, country: string) {
  const tasks = createTaskFlow(apicServices);
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

export async function collect(options: Options) {
  try {
    let proxy_list: any[] = [];

    if (options.algorithm === 'apic') {
      proxy_list = await APIc(options.protocol, options.country);
    } else if (options.algorithm === 'upc') {
      proxy_list = await UPc(options.protocol);
    }

    if (options.count !== 'max') {
      proxy_list = proxy_list.slice(0, Number(options.count));
    }

    proxy_list = Array.from(new Set(proxy_list.map(String)));

    const out = proxy_list.join('\n');

    return out;
  } catch (err: any) {
    return err;
  }
}