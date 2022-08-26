use bevy_rapier2d::geometry::CollisionGroups;
use bevy_rapier2d::prelude::InteractionGroups;

pub const WALL_BIT: u32 = 1 << 0;
pub const COMBOBOX_BIT: u32 = 1 << 1;
pub const PLAYER_BIT: u32 = 1 << 2;
pub const ELEVATOR_BIT: u32 = 1 << 3;

pub const WALL_FILTER: u32 = COMBOBOX_BIT | PLAYER_BIT;
pub const COMBOBOX_FILTER: u32 = WALL_BIT | PLAYER_BIT | ELEVATOR_BIT | COMBOBOX_BIT;
pub const PLAYER_FILTER: u32 = WALL_BIT | COMBOBOX_BIT | ELEVATOR_BIT | PLAYER_BIT;
pub const ELEVATOR_FILTER: u32 = PLAYER_BIT | COMBOBOX_BIT;

pub const WALL: CollisionGroups = CollisionGroups::new(WALL_BIT, WALL_FILTER);
pub const COMBOBOX: CollisionGroups = CollisionGroups::new(COMBOBOX_BIT, COMBOBOX_FILTER);
pub const PLAYER: CollisionGroups = CollisionGroups::new(PLAYER_BIT, PLAYER_FILTER);
pub const ELEVATOR: CollisionGroups = CollisionGroups::new(ELEVATOR_BIT, ELEVATOR_FILTER);

pub const WALL_I: InteractionGroups = InteractionGroups::new(WALL_BIT, WALL_FILTER);
pub const COMBOBOX_I: InteractionGroups = InteractionGroups::new(COMBOBOX_BIT, COMBOBOX_FILTER);
pub const PLAYER_I: InteractionGroups = InteractionGroups::new(PLAYER_BIT, PLAYER_FILTER);
pub const ELEVATOR_I: InteractionGroups = InteractionGroups::new(ELEVATOR_BIT, ELEVATOR_FILTER);
