import { app } from './listeners.js';

const ports = [37621, 32971, 34293, 37640];

export function setup() {
  for (const port of ports) {
    const operation = app.listen(port);

    if (operation) {
      return port;
    }
  }

  return null;
}