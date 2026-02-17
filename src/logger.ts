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

  if (type === 'system') {
    line.className += ' log-line-system';
  }

  if (type === 'extended') {
    line.className += ' log-line-extended';
    line.style.fontStyle = 'italic';
  }

  line.innerHTML = `
    <div class="log-line-date">${date()}</div>
    <div class="log-line-content ${type}">${text}</div>
  `;

  if ((line.className.includes('log-line-system') && !statistics.visible.system) || (line.className.includes('log-line-extended') && !statistics.visible.extended)) {
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

  document.querySelectorAll<HTMLElement>(`.log-line-${type}`).forEach(e => {
    e.style.display = state ? 'flex' : 'none';
  });
}

export function eraseLogs(): void {
  const logs = document.querySelectorAll('.log-line');
  logs.forEach(e => e.remove());
  statistics.index = 0;
}