use crate::base::bot::*;
use crate::base::*;
use crate::emit::*;

/// Вспомогательная функция отправки сообщения в чат от бота
pub fn send_message_from_bot(username: String, message: String) {
  tokio::spawn(async move {
    BOT_REGISTRY
      .get_bot(&username, async |bot| {
        bot.chat(message);
      })
      .await;
  });
}

/// Вспомогательная функция, позволяющая сбросить все задачи и состояния бота
pub fn reset_bot(username: String) {
  tokio::spawn(async move {
    BOT_REGISTRY
      .get_bot(&username, async |bot| {
        TASKS.reset(&username);
        STATES.reset(&username);

        bot.set_crouching(false);
        bot.set_jumping(false);

        send_log(
          format!("Все задачи и состояния бота {} сброшены", username),
          "info",
        );
        send_message(
          "Система",
          format!("Все задачи и состояния бота {} сброшены", username),
        );
      })
      .await;
  });
}

/// Вспомогательная функция отключения бота
pub fn disconnect_bot(username: String) {
  tokio::spawn(async move {
    TASKS.reset(&username);
    PLUGIN_MANAGER.destroy_all_tasks(&username);

    if let Some(bot) = BOT_REGISTRY.take_bot(&username).await {
      bot.disconnect();
    }

    PROFILES.set_bool(&username, "captcha_caught", false);
    STATES.reset(&username);
    TASKS.remove(&username);
    PROFILES.set_str(&username, "status", "Оффлайн");

    send_log(format!("Бот {} отключился", username), "info");
    send_message("Система", format!("Бот {} отключился", username));
  });
}
