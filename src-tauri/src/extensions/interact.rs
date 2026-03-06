use azalea::{
  core::direction::Direction,
  protocol::packets::game::{
    s_interact::InteractionHand, s_player_action::Action, ServerboundPlayerAction,
    ServerboundUseItem,
  },
  BlockPos, Client,
};

pub trait BotInteractExt {
  fn start_use_held_item(&self, hand: InteractionHand);
  fn release_use_held_item(&self);
}

impl BotInteractExt for Client {
  fn start_use_held_item(&self, hand: InteractionHand) {
    let direction = self.direction();

    self.write_packet(ServerboundUseItem {
      hand: hand,
      y_rot: direction.0,
      x_rot: direction.1,
      seq: 0,
    });
  }

  fn release_use_held_item(&self) {
    self.write_packet(ServerboundPlayerAction {
      action: Action::ReleaseUseItem,
      pos: BlockPos::new(0, 0, 0),
      direction: Direction::Down,
      seq: 0,
    });
  }
}
