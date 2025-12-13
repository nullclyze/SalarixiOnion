import { SocksClient } from 'socks';
import net from 'net';

async function connection(options: any) {
  return new Promise((resolve, reject) => {
    if (options.type === 'socks5' || options.type === 'socks4') {
      let data: any;

      if (options.username && options.password) {
        data = {
          host: options.host,
          port: options.port,
          type: options.type === 'socks5' ? 5 : 4,
          userId: options.username,
          password: options.password
        };
      } else {
        data = {
          host: options.host,
          port: options.port,
          type: options.type === 'socks5' ? 5 : 4
        };
      }

      SocksClient.createConnection({
        proxy: data,
        timeout: options.timeout,
        command: 'connect',
        destination: {
          host: options.address.split(':')[0],
          port: parseInt(options.address.split(':')[1])
        }
      }, 
      (error, info) => {
        if (error) return reject(error.message);
        
        resolve(info?.socket);
      });
    } else {
      const socket = net.connect({
				host: options.host,
			  port: options.port
			});
						
			socket.on('connect', () => {
				try {
					let req = `CONNECT ${options.address.split(':')[0]}:${parseInt(options.address.split(':')[1])} HTTP/1.1\r\n` +
                    `Host: ${options.address.split(':')[0]}:${parseInt(options.address.split(':')[1])}\r\n`;
          
          if (options.username && options.password) {
            req += `Proxy-Authorization: Basic ${Buffer.from(`${options.username}:${options.password}`).toString('base64')}\r\n` + `\r\n`;
          }

					socket.write(req);

					let res = '';
							
					function handler(chunk: Buffer) {
						res += chunk.toString();

						if (res.includes('200')) {
							socket.removeListener('data', handler);
							const payload = res.split('\r\n\r\n')[1] || '';

							if (payload) console.log('There is extra options left in the buffer:', payload);

              resolve(socket);
						} else if (res.includes('\r\n\r\n')) {
							console.log('Proxy returned an error:', res.trim());
							socket.destroy();
              reject(new Error('Proxy returned an error:' + res.trim()));
						}
					}
									
					socket.on('data', handler);
				} catch (error) {
					reject(error);
				}
			});

      socket.setTimeout(options.timeout, () => {
        socket.destroy();
        reject(new Error('Proxy connection timeout'));
      });

      socket.on('close', () => {
        reject(new Error('Proxy connection closed'));
      });

			socket.on('error', (error) => {
				console.log('Proxy error:', error);
				reject(error);
			});
    }
  });
}

export { connection };