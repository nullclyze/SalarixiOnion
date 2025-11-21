import date from './tools/date';

let LOG_INDEX = 0;

// Класс для вывода логов в интерфейс клиента 
class Logger {
  private replace(text: string): string {
    return text
      .replace(/%hcg/g, '<span style="color: #21d618ba;">')
      .replace(/%hcy/g, '<span style="color: #d6d018b6;">')
      .replace(/%hcr/g, '<span style="color: #d61b1893;">')
      .replace(/%sc/g, '</span>');
  }

  log(message: string, type: string): void {
    const logContent = document.getElementById('log-content') as HTMLElement;

    if (LOG_INDEX >= 300) {
      LOG_INDEX = 0;
      logContent.innerHTML = '';
    }

    LOG_INDEX++;

    const container = document.createElement('div');

    container.className = 'log-line';

    if (type === 'log-system') {
      container.className += ' log-line-system';
    }

    container.innerHTML = `
      <div class="log-line-number">${LOG_INDEX}</div>
      <div class="log-line-content ${type}"><span class="log-timestamp">(${date()})</span> ${this.replace(message)}</div>
    `;

    logContent.appendChild(container);

    if (LOG_INDEX > 10) {
      logContent.scrollTo({
        top: logContent.scrollHeight,
        behavior: 'smooth'
      });
    }
  }
}

export default Logger;