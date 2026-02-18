import { generateId } from './helpers/generate';

let showMessages = true;
let messageCounter = 0;

function removeMessage(id: string): void {
  document.getElementById(id)?.remove();
}

export function changeMessagesVisibility(state: string): void {
  if (state === 'show') {
    showMessages = true;
  } else {
    showMessages = false;
  }
}

export function spawnMessage(name: string, content: string): void {
  const wrap = document.getElementById('messages-wrap');

  if (wrap && showMessages && messageCounter <= 2) {
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

    messageCounter++;

    wrap.appendChild(msg);

    let removed = false;

    const listener = () => {
      removed = true;

      msg.classList.add('hide');

      setTimeout(() => {
        removeMessage(msgId);
        messageCounter -= 1;
      }, 200);
    }

    document.getElementById(closeBtnId)?.addEventListener('click', listener);

    setTimeout(() => {
      msg.classList.add('hide');
    }, 3700);

    setTimeout(() => {
      document.getElementById(closeBtnId)?.removeEventListener('click', listener);
      removeMessage(msgId);

      if (!removed) {
        messageCounter -= 1;
      }
    }, 3900);
  }
}