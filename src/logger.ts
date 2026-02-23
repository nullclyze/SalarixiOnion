import { date } from './helpers/date';

interface LoggerStatistics {
  index: number;
  visible: {
    system: boolean;
    extended: boolean;
  };
}

const statistics: LoggerStatistics = {
  index: 0,
  visible: {
    system: true,
    extended: false
  }
};

export function log(text: string, type: string): void {
  const journal = document.getElementById('log-content') as HTMLElement;

  if (!journal) return;

  if (statistics.index >= 400) {
    statistics.index = 399;
    journal.firstChild?.remove();
  }

  statistics.index++;

  const line = document.createElement('div');

  line.className = 'log-line';

  if (type === 'system' || type === 'extended') {
    line.setAttribute('log-type', type);
  }

  line.innerHTML = `
    <div class="log-line-date">${date()}</div>
  `;

  const lineContent = document.createElement('div');
  lineContent.className = `log-line-content ${type}`;
  lineContent.innerText = text;
  
  if (type === 'extended') {
    lineContent.style.fontStyle = 'italic';
  }

  line.appendChild(lineContent);

  if ((line.getAttribute('log-type') === 'system' && !statistics.visible.system) || (line.getAttribute('log-type') === 'extended' && !statistics.visible.extended)) {
    line.style.display = 'none';
  }

  journal.appendChild(line);

  if ((document.getElementById('auto-scroll-log') as HTMLInputElement).checked) {
    journal.scrollTo({
      top: journal.scrollHeight,
      behavior: 'smooth'
    });
  }
}

export function changeLogsVisibility(type: 'system' | 'extended', state: boolean): void {
  statistics.visible[type] = state;

  document.querySelectorAll<HTMLElement>(`[log-type="${type}"]`).forEach(e => {
    e.style.display = state ? 'flex' : 'none';
  });
}

export function eraseLogs(): void {
  const logs = document.querySelectorAll('.log-line');
  logs.forEach(e => e.remove());
  statistics.index = 0;
}