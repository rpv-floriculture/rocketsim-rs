use super::{CarState, BallState};

#[derive(Clone, Debug, Default)]
pub struct GameState {
    pub car_states: Vec<CarState>,
    pub ball_state: BallState,
}