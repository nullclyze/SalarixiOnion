import { date } from './date';

interface LoggerStatistics {
  index: number;
  visible: {
    system: boolean;
    extended: boolean;
  };
}

class Logger {
  private journal: HTMLDivElement | null;
  private statistics: LoggerStatistics;

  constructor() {
    this.journal = null;
    this.statistics = {
      index: 0,
      visible: {
        system: true,
        extended: false
      }
    };
  }

  /** Метод инициализации логгера и связанных с ним функций. */
  public init(): void {
    this.journal = document.getElementById('log-content') as HTMLDivElement;
    document.getElementById('clear-journal')?.addEventListener('click', () => this.clear());
  }

  /** 
   * Метод отправки сообщения в журнал.
   * 
   * Данный метод проверяет количество существующих сообщений в журнале.
   * Если количество равняется или превышает 400, то удаляется самое старое сообщение из журнала.
   * 
   * Ещё этот метод учитывает текущие состояния видимости определённых типов сообщений.
   *
   * Так же при включенной опции "Авто скролл" этот метод будет каждое новое сообщение прокручивать контент до самого низа.
   */
  public log(text: string, type: string): void {
    if (!this.journal) return;

    if (this.statistics.index >= 400) {
      this.journal.firstChild?.remove();
      this.statistics.index = 399;
    }

    this.statistics.index++;

    const line = document.createElement('div');
    line.className = 'log-line';
    if (type === 'system' || type === 'extended') line.setAttribute('log-type', type);

    line.innerHTML = `
      <div class="log-line-date">${date()}</div>
    `;

    const content = document.createElement('div');
    content.className = `log-line-content ${type}`;
    content.innerText = text;
    
    if (type === 'extended') content.style.fontStyle = 'italic';
    if ((line.getAttribute('log-type') === 'system' && !this.statistics.visible.system) || (line.getAttribute('log-type') === 'extended' && !this.statistics.visible.extended)) line.style.display = 'none';

    line.appendChild(content);
    this.journal.appendChild(line);

    if ((document.getElementById('journal_chbx_auto-scroll') as HTMLInputElement).checked) this.journal.scrollTo({ top: this.journal.scrollHeight, behavior: 'smooth' });
  }

  /** Метод смены видимости определённых типов сообщений в журнале. */
  public setVisibility(type: 'system' | 'extended', state: boolean): void {
    this.statistics.visible[type] = state;
    document.querySelectorAll<HTMLElement>(`[log-type="${type}"]`).forEach(e => e.style.display = state ? 'flex' : 'none');
  }

  /** Метод полной очистки журнала. */
  private clear(): void {
    document.querySelectorAll('.log-line').forEach(e => e.remove());
    this.statistics.index = 0;
  }
}

const logger = new Logger();

export { logger }