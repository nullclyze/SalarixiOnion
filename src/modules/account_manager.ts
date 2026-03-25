import { open } from '@tauri-apps/plugin-dialog';

import { date } from '../utils/date';
import { logger } from '../utils/logger';
import { readFile, writeFile } from '@tauri-apps/plugin-fs';
import { join } from '@tauri-apps/api/path';
import { messages } from '../utils/message';

type AccountFields = {
  creation_date: string;
  password: string | null;
  proxy: string | null;
  proxy_username: string | null;
  proxy_password: string | null;
}

class AccountManager {
  private accountList: HTMLDivElement | null = null;
  private accountCounter: HTMLSpanElement | null = null;
  private wrappers: HTMLDivElement | null = null;

  private accounts: Record<string, AccountFields | null> = {};

  /** Метод инициализации функций, связанных с аккаунтами. */
  public init(): void {
    this.accountList = document.getElementById('account-list') as HTMLDivElement;
    this.accountCounter = document.getElementById('account-counter') as HTMLSpanElement;
    this.wrappers = document.getElementById('account-wrappers') as HTMLDivElement;

    this.loadSavedAccounts();

    const addAccountBtn = document.getElementById('add-account') as HTMLButtonElement;
    const removeAllAccountsBtn = document.getElementById('remove-all-accounts') as HTMLButtonElement;
    const importAccounts = document.getElementById('import-accounts') as HTMLButtonElement;
    const exportAccounts = document.getElementById('export-accounts') as HTMLButtonElement;

    addAccountBtn.addEventListener('click', () => {
      const usernameInput = document.getElementById('account-username') as HTMLInputElement;
      const username = usernameInput.value;

      if (this.accounts[username] || username === '') return;

      usernameInput.value = '';

      this.createAccountCard(username, {
        creation_date: date('exact'),
        password: null,
        proxy: null,
        proxy_username: null,
        proxy_password: null
      });

      this.updateAccountCounter();
    });

    removeAllAccountsBtn.addEventListener('click', () => this.clearAccounts());

    importAccounts.addEventListener('click', async () => {
      try {
        const path = await open({
          directory: false,
          multiple: false,
          filters: [{
            name: 'Accounts',
            extensions: ['json']
          }]
        });

        if (!path) return;

        this.clearAccounts();

        const buffer = await readFile(path);

        const decoder = new TextDecoder();
        const accounts = JSON.parse(decoder.decode(buffer));

        for (const username in accounts) {
          const data = accounts[username];
          if (!data) continue;
          this.createAccountCard(username, {
            creation_date: date('exact'),
            password: data.password,
            proxy: data.proxy,
            proxy_username: data.proxy_username,
            proxy_password: data.proxy_password
          });
        } 
        
        this.updateAccountCounter();

        messages.message('Импорт аккаунтов', `Аккаунты успешно импортированы`);
      } catch (error) {
        logger.log(`Ошибка импорта аккаунтов: ${error}`, 'error');
      }
    });

    exportAccounts.addEventListener('click', async () => {
      try {
        const directory = await open({
          directory: true,
          multiple: false
        });

        if (!directory) return;

        const accounts: any = {};

        for (const username in this.accounts) {
          const data = this.accounts[username];
          if (!data) continue;
          accounts[username] = {
            password: data.password,
            proxy: data.proxy,
            proxy_username: data.proxy_username,
            proxy_password: data.proxy_password
          };
        }

        const path = await join(directory, 'salarixi.accounts.json');
        let encoder = new TextEncoder();
        let buffer = encoder.encode(JSON.stringify(accounts, null, 2));

        await writeFile(path, buffer);

        messages.message('Экспорт аккаунтов', `Аккаунты успешно экспортированы`);
      } catch (error) {
        logger.log(`Ошибка экспорта аккаунтов: ${error}`, 'error');
      }
    });

    setInterval(() => localStorage.setItem('salarixionion:storage:accounts', JSON.stringify(this.accounts)), 1000);
  }

  /** Метод получения всех существующих аккаунтов и их опций. */
  public getAccounts(): Record<string, AccountFields | null> {
    return this.accounts;
  }

  /** Метод обновления счётчика аккаунтов. */
  private updateAccountCounter(): void {
    let count = 0;
    for (const _ in this.accounts) count++;
    if (this.accountCounter) this.accountCounter.innerText = count.toString();
  }

  /** Метод полной очистки всех аккаунтов. */
  private clearAccounts(): void {
    document.querySelectorAll('[wrapper="account-editor"]').forEach(w => w.remove());
    document.querySelectorAll('[wrapper="account-card"]').forEach(w => w.remove());
    for (const username in this.accounts) document.getElementById(`account-${username}`)?.remove();
    this.accounts = {};
    localStorage.removeItem('salarixionion:storage:accounts');
    this.updateAccountCounter();
  }

  /** Метод загрузки сохранённых аккаунтов из локального хранилища. */
  private loadSavedAccounts(): void {
    const accounts = localStorage.getItem('salarixionion:storage:accounts');
    if (!accounts) return;
    for (const [username, fields] of Object.entries<AccountFields | null>(JSON.parse(accounts))) fields ? this.createAccountCard(username, fields) : null;
    this.updateAccountCounter();
  }

  /** Метод создания карточки аккаунта. */
  private createAccountCard(username: string, fields: AccountFields): void {
    this.accounts[username] = {
      creation_date: fields.creation_date,
      password: fields.password,
      proxy: fields.proxy,
      proxy_username: fields.proxy_username,
      proxy_password: fields.proxy_password
    };

    const row = document.createElement('div');
    row.className = 'account-row';
    row.id = `account-${username}`;
    row.setAttribute('wrapper', 'account-card');

    row.innerHTML = `
      <div class="account-cell username-cell">${username.substring(0, 16)}</div>
      <div class="account-cell date-cell">${fields.creation_date}</div>
      <div class="account-cell actions-cell">
        <button class="action-btn" id="edit-account-${username}" title="Настройки">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16">
            <path d="M8 4.754a3.246 3.246 0 1 0 0 6.492 3.246 3.246 0 0 0 0-6.492M5.754 8a2.246 2.246 0 1 1 4.492 0 2.246 2.246 0 0 1-4.492 0"/>
            <path d="M9.796 1.343c-.527-1.79-3.065-1.79-3.592 0l-.094.319a.873.873 0 0 1-1.255.52l-.292-.16c-1.64-.892-3.433.902-2.54 2.541l.159.292a.873.873 0 0 1-.52 1.255l-.319.094c-1.79.527-1.79 3.065 0 3.592l.319.094a.873.873 0 0 1 .52 1.255l-.16.292c-.892 1.64.901 3.434 2.541 2.54l.292-.159a.873.873 0 0 1 1.255.52l.094.319c.527 1.79 3.065 1.79 3.592 0l.094-.319a.873.873 0 0 1 1.255-.52l.292.16c1.64.893 3.434-.902 2.54-2.541l-.159-.292a.873.873 0 0 1 .52-1.255l.319-.094c1.79-.527 1.79-3.065 0-3.592l-.319-.094a.873.873 0 0 1-.52-1.255l.16-.292c.893-1.64-.902-3.433-2.541-2.54l-.292.159a.873.873 0 0 1-1.255-.52zm-2.633.283c.246-.835 1.428-.835 1.674 0l.094.319a1.873 1.873 0 0 0 2.693 1.115l.291-.16c.764-.415 1.6.42 1.184 1.185l-.159.292a1.873 1.873 0 0 0 1.116 2.692l.318.094c.835.246.835 1.428 0 1.674l-.319.094a1.873 1.873 0 0 0-1.115 2.693l.16.291c.415.764-.42 1.6-1.185 1.184l-.291-.159a1.873 1.873 0 0 0-2.693 1.116l-.094.318c-.246.835-1.428.835-1.674 0l-.094-.319a1.873 1.873 0 0 0-2.692-1.115l-.292.16c-.764.415-1.6-.42-1.184-1.185l.159-.291A1.873 1.873 0 0 0 1.945 8.93l-.319-.094c-.835-.246-.835-1.428 0-1.674l.319-.094A1.873 1.873 0 0 0 3.06 4.377l-.16-.292c-.415-.764.42-1.6 1.185-1.184l.292.159a1.873 1.873 0 0 0 2.692-1.115z"/>
          </svg>
        </button>
        <button class="action-btn" id="remove-account-${username}" title="Удалить">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16">
            <path d="M2.5 1a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1H3v9a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4h.5a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1zm3 4a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 .5-.5M8 5a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7A.5.5 0 0 1 8 5m3 .5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 1 0"/>
          </svg>
        </button>
      </div>
    `;

    if (!this.accountList) return;

    this.accountList.appendChild(row);

    this.initializeAccountCard(row, username, fields);
  }

  /** Метод создания обёркти редактора настроек аккаунта. */
  private createEditWrapper(username: string): HTMLDivElement {
    const wrapper = document.createElement('div');
    wrapper.className = 'cover';
    wrapper.id = `account-editor-${username}`;
    wrapper.setAttribute('wrapper', 'account-editor');

    wrapper.innerHTML = `
      <div class="panel with-header" style="margin-bottom: 20px;">
        <div class="left">
          <div class="header">Настройки аккаунта ${username}</div>
        </div>

        <div class="right">
          <button class="btn min" id="close-account-editor-${username}">
            ⨉
          </button>
        </div>
      </div>

      <div style="display: flex; flex-direction: column; gap: 10px;">
        <div class="pretty-input-wrapper">
          <p class="signature fix">Пароль</p>
          <input type="text" id="account-password-${username}" placeholder="qwerty12345">
        </div>

        <div class="pretty-input-wrapper">
          <p class="signature fix">Прокси</p>
          <input type="text" id="account-proxy-${username}" placeholder="socks5://35.91.83.91:1111">
        </div>

        <div class="pretty-input-wrapper">
          <p class="signature fix">Юзернейм прокси</p>
          <input type="text" id="account-proxy-username-${username}" placeholder="user">
        </div>

        <div class="pretty-input-wrapper">
          <p class="signature fix">Пароль прокси</p>
          <input type="text" id="account-proxy-password-${username}" placeholder="root">
        </div>
      </div>
    `;

    this.wrappers?.appendChild(wrapper);

    return wrapper;
  }

  /** Метод инициализации карточки аккаунта. */
  private initializeAccountCard(row: HTMLDivElement, username: string, fields: AccountFields): void {
    try {
      const editorWrapper = this.createEditWrapper(username);

      document.getElementById(`edit-account-${username}`)?.addEventListener('click', () => editorWrapper.style.display = 'flex');
      document.getElementById(`close-account-editor-${username}`)?.addEventListener('click', () => editorWrapper.style.display = 'none');

      document.getElementById(`remove-account-${username}`)?.addEventListener('click', () => {
        row.remove();
        editorWrapper.remove();
        delete this.accounts[username];

        this.updateAccountCounter();
      });

      const account = this.accounts[username];

      if (!account) return;

      const passwordInput = document.getElementById(`account-password-${username}`) as HTMLInputElement;
      const proxyInput = document.getElementById(`account-proxy-${username}`) as HTMLInputElement;
      const proxyUsernameInput = document.getElementById(`account-proxy-username-${username}`) as HTMLInputElement;
      const proxyPasswordInput = document.getElementById(`account-proxy-password-${username}`) as HTMLInputElement;

      fields.password ? passwordInput.value = fields.password : null;
      fields.proxy ? proxyInput.value = fields.proxy : null;
      fields.proxy_username ? proxyUsernameInput.value = fields.proxy_username : null;
      fields.proxy_password ? proxyPasswordInput.value = fields.proxy_password : null;

      passwordInput.addEventListener('input', () => account.password = passwordInput.value);
      proxyInput.addEventListener('input', () => account.proxy = proxyInput.value);
      proxyUsernameInput.addEventListener('input', () => account.proxy_username = proxyUsernameInput.value);
      proxyPasswordInput.addEventListener('input', () => account.proxy_password = proxyPasswordInput.value);
    } catch (error) {
      logger.log(`Ошибка инициализации аккаунта ${username}: ${error}`, 'error');
    }
  }
}

const accountManager = new AccountManager();

export { accountManager }