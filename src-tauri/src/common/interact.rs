use azalea::core::direction::Direction;
use azalea::core::position::BlockPos;
use azalea::prelude::*;
use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::protocol::packets::game::s_player_action::Action;
use azalea::protocol::packets::game::{ServerboundPlayerAction, ServerboundUseItem};

/// Функция отправки пакета StartUseItem
pub fn start_use_item(bot: &Client, hand: InteractionHand) {
  let direction = bot.direction();

  bot.write_packet(ServerboundUseItem {
    hand: hand,
    y_rot: direction.0,
    x_rot: direction.1,
    seq: 0,
  });
}

/// Функция отправки пакета ReleaseUseItem
pub fn release_use_item(bot: &Client) {
  bot.write_packet(ServerboundPlayerAction {
    action: Action::ReleaseUseItem,
    pos: BlockPos::new(0, 0, 0),
    direction: Direction::Down,
    seq: 0,
  });
}
