pub const SOCCAR: i32 = 0;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum GameMode {
    Soccar
}

impl TryFrom<i32> for GameMode {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GameMode::Soccar),
            _ => Err(()),
        }
    }
}

impl From<GameMode> for i32 {
    fn from(value: GameMode) -> Self {
        match value {
            GameMode::Soccar => 0,
        }
    }
}

impl Default for GameMode {
    fn default() -> Self {
        GameMode::Soccar
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum Team {
    Blue,
    Orange
}

impl TryFrom<i32> for Team {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Team::Blue),
            1 => Ok(Team::Orange),
            _ => Err(()),
        }
    }
}

impl From<Team> for i32 {
    fn from(value: Team) -> Self {
        match value {
            Team::Blue => 0,
            Team::Orange => 1,
        }
    }
}

impl Default for Team {
    fn default() -> Self {
        Team::Blue
    }
}