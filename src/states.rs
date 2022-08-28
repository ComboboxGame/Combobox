#[derive(Clone, PartialEq, Eq, Debug, Hash, Copy)]
pub enum GuiState {
    None,
    MainScreen,
    LevelSelection,
    Level,
    LevelCompleted,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Copy)]
pub enum AudioState {
    None,
    Menu,
    Level,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Copy)]
pub enum LevelState {
    None,
    Level,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Copy)]
pub enum CameraState {
    None,
    FollowPlayers,
}