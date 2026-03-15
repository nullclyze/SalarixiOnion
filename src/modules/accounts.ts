import { open } from '@tauri-apps/plugin-dialog';

import { date } from '../helpers/date';
import { log } from '../logger';
import { readFile, writeFile } from '@tauri-apps/plugin-fs';
import { path } from '@tauri-apps/api';
import { message } from '../message';

type AccountFields = {
  creation_date: string;
  password: string | null;
  proxy: string | null;
  proxy_username: string | null;
  proxy_password: string | null;
}

export class AccountManager {
  private accountList: HTMLDivElement | null = null;
  private accountCounter: HTMLSpanElement | null = null;
  private wrappers: HTMLDivElement | null = null;

  private accounts: Record<string, AccountFields | null> = {};

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
          filters: [
            {
              name: 'Accounts',
              extensions: ['json']
            }
          ]
        });

        if (path) {
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

          message('Импорт аккаунтов', `Аккаунты успешно импортированы`);
        }
      } catch (error) {
        log(`Ошибка импорта аккаунтов: ${error}`, 'error');
      }
    });

    exportAccounts.addEventListener('click', async () => {
      try {
        const directory = await open({
          directory: true,
          multiple: false
        });

        if (directory) {
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

          let encoder = new TextEncoder();
          let data = encoder.encode(JSON.stringify(accounts, null, 2));

          await writeFile(await path.join(directory, 'salarixi.accounts.json'), data);

          message('Экспорт аккаунтов', `Аккаунты успешно экспортированы`);
        }
      } catch (error) {
        log(`Ошибка экспорта аккаунтов: ${error}`, 'error');
      }
    });

    setInterval(() => {
      localStorage.setItem('salarixionion:storage:accounts', JSON.stringify(this.accounts));
    }, 1000);
  }

  public getAccounts(): Record<string, AccountFields | null> {
    return this.accounts;
  }

  private updateAccountCounter(): void {
    let count = 0;

    for (const _ in this.accounts) {
      count++;
    }

    if (this.accountCounter) {
      this.accountCounter.innerText = count.toString();
    }
  }

  private clearAccounts(): void {
    document.querySelectorAll('[account-editor="true"]').forEach(e => e.remove());

    for (const username in this.accounts) {
      const card = document.getElementById(`account-${username}`) as HTMLDivElement;
      card.remove();
    }

    this.accounts = {};

    localStorage.removeItem('salarixionion:storage:accounts');

    this.updateAccountCounter();
  }

  private loadSavedAccounts(): void {
    const accounts = localStorage.getItem('salarixionion:storage:accounts');

    if (accounts) {
      const json = JSON.parse(accounts);

      for (const [username, fields] of Object.entries<AccountFields | null>(json)) {
        if (fields) {
          this.createAccountCard(username, fields);
        }
      }

      this.updateAccountCounter();
    }
  }

  private createAccountCard(username: string, fields: AccountFields): void {
    this.accounts[username] = {
      creation_date: fields.creation_date,
      password: fields.password,
      proxy: fields.proxy,
      proxy_username: fields.proxy_username,
      proxy_password: fields.proxy_password
    };

    const card = document.createElement('div');
    card.className = 'account';
    card.id = `account-${username}`;

    card.innerHTML = `
      <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-person-fill" viewBox="0 0 16 16">
        <path d="M3 14s-1 0-1-1 1-4 6-4 6 3 6 4-1 1-1 1zm5-6a3 3 0 1 0 0-6 3 3 0 0 0 0 6"/>
      </svg>

      <div class="text">
        <p class="username">${username}</p>
        <p class="creation-date">Создан: ${fields.creation_date}</p>
      </div>

      <div class="btn-group">
        <button class="btn min pretty" id="edit-account-${username}">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-pencil-square" viewBox="0 0 16 16">
            <path d="M15.502 1.94a.5.5 0 0 1 0 .706L14.459 3.69l-2-2L13.502.646a.5.5 0 0 1 .707 0l1.293 1.293zm-1.75 2.456-2-2L4.939 9.21a.5.5 0 0 0-.121.196l-.805 2.414a.25.25 0 0 0 .316.316l2.414-.805a.5.5 0 0 0 .196-.12l6.813-6.814z"/>
            <path fill-rule="evenodd" d="M1 13.5A1.5 1.5 0 0 0 2.5 15h11a1.5 1.5 0 0 0 1.5-1.5v-6a.5.5 0 0 0-1 0v6a.5.5 0 0 1-.5.5h-11a.5.5 0 0 1-.5-.5v-11a.5.5 0 0 1 .5-.5H9a.5.5 0 0 0 0-1H2.5A1.5 1.5 0 0 0 1 2.5z"/>
          </svg>
        </button>

        <button class="btn min pretty" id="remove-account-${username}">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash-fill" viewBox="0 0 16 16">
            <path d="M2.5 1a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1H3v9a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4h.5a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1zm3 4a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 .5-.5M8 5a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7A.5.5 0 0 1 8 5m3 .5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 1 0"/>
          </svg>
        </button>
      </div>
    `;

    this.accountList?.appendChild(card);

    this.initializeAccountCard(card, username, fields);
  }

  private createEditWrapper(username: string): HTMLDivElement {
    const wrapper = document.createElement('div');
    wrapper.className = 'cover';
    wrapper.id = `account-editor-${username}`;
    wrapper.setAttribute('account-editor', 'true');

    wrapper.innerHTML = `
      <div class="panel with-header" style="margin-bottom: 20px;">
        <div class="left">
          <div class="header">Настройки аккаунта ${username}</div>
        </div>

        <div class="right">
          <button class="btn min pretty" id="close-account-editor-${username}">
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

  private initializeAccountCard(card: HTMLDivElement, username: string, fields: AccountFields): void {
    try {
      const editorWrapper = this.createEditWrapper(username);

      document.getElementById(`edit-account-${username}`)?.addEventListener('click', () => {
        editorWrapper.style.display = 'flex';
      });

      document.getElementById(`close-account-editor-${username}`)?.addEventListener('click', () => {
        editorWrapper.style.display = 'none';
      });

      document.getElementById(`remove-account-${username}`)?.addEventListener('click', () => {
        card.remove();
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
      log(`Ошибка инициализации аккаунта ${username}: ${error}`, 'error');
    }
  }
}