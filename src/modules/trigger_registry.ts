class TriggerRegistry {
  private triggers: Record<string, Function> = {};

  /** Метод регистрации триггер-функции на определёный элемент и его событие. */
  public register(id: string, type: 'checkbox' | 'select', fn: Function): void {
    const doc = document.getElementById(id);
    const el = type === 'select' ? doc as HTMLSelectElement : doc as HTMLInputElement;
    const event = type === 'select' ? 'change' : 'input';
    el.addEventListener(event, () => fn(el));
    this.triggers[id] = fn;
  }

  /** Метод триггеринга всех зарегистрированных функций. */
  public triggerAll(): void {
    document.querySelectorAll<HTMLElement>('[trigger]').forEach(e => this.invoke(e.id));
  } 

  /** Метод вызова определённой зарегистрированной функции. */
  private invoke(id: string): void {
    for (const triggerId in this.triggers) {
      if (triggerId !== id) continue;
      const el = document.getElementById(id);
      if (!el) continue;
      this.triggers[triggerId](el);
    } 
  }
}

const triggerRegistry = new TriggerRegistry();

export { triggerRegistry }