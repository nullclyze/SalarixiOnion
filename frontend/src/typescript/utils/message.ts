import { generateId } from './generator';

class Messages {
  private wrapper: HTMLDivElement | null = null;
  private show: boolean = true;
  private counter: number = 0;

  /** Метод инициализации обёртки сообщений. */
  public init(): void {
    this.wrapper = document.getElementById('messages-wrapper') as HTMLDivElement;
  }

  /** Метод отправки определённого сообщения. */
  public message(name: string, content: string): void {
    if (!this.wrapper) return;

    if (!this.show || this.counter > 2) return;

    const msgId = `msg-${generateId()}`;
    const closeBtnId = `close-msg-btn-${generateId()}`;

    const msg = document.createElement('div');
    msg.className = 'message';
    msg.id = msgId;

    msg.innerHTML = `
      <div class="up-part">
        <div class="name">${name}</div>
        <button class="close-btn" id="${closeBtnId}">✕</button>
      </div>

      <div class="down-part">${content}</div>

      <div class="progress-bar"></div>
    `;

    this.counter++;
    this.wrapper.appendChild(msg);

    let removed = false;

    const listener = (): void => {
      removed = true;
      msg.classList.add('hide');
      setTimeout(() => {
        this.remove(msgId);
        this.counter -= 1;
      }, 200);
    }

    document.getElementById(closeBtnId)?.addEventListener('click', listener);

    setTimeout(() => msg.classList.add('hide'), 3700);

    setTimeout(() => {
      document.getElementById(closeBtnId)?.removeEventListener('click', listener);
      this.remove(msgId);
      if (!removed) this.counter -= 1;
    }, 3900);
  }

  /** Метод изменения видимости сообщений. */
  public visibility(state: string): void {
    this.show = state === 'show';
  }

  /** Метод удаления определённого сообщения. */
  private remove(id: string): void {
    document.getElementById(id)?.remove();
  }
}

const messages = new Messages();

export { messages }