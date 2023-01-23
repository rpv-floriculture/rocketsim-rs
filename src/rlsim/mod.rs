mod ball_state;
mod car_state;
mod constants;
mod game_state;

use std::mem::MaybeUninit;

pub use self::{
    ball_state::BallState, car_state::CarState, car_state::CarStateBuilder, game_state::GameState,
};
pub use constants::*;

type ArenaRef = *const libc::c_void;
pub struct Arena {
    _ref: ArenaRef,
    car_ids: Vec<u32>,
}
unsafe impl Send for Arena {}
unsafe impl Sync for Arena {}

#[link(name = "RocketSimC")]
extern "C" {
    fn CreateArena(gameMode: i32) -> ArenaRef;
    fn DestroyArena(arena: ArenaRef);
    fn AddCar(arena: ArenaRef, team: i32) -> u32;
    fn RemoveCar(arena: ArenaRef, carId: u32) -> bool;
    fn Controls(
        arena: ArenaRef,
        carId: u32,
        throttle: f32,
        steer: f32,
        pitch: f32,
        yaw: f32,
        roll: f32,
        boost: bool,
        jump: bool,
        handbrake: bool,
    ) -> bool;
    fn GetCarState(arena: ArenaRef, carId: u32, target: *mut CarState) -> bool;
    fn SetCarState(arena: ArenaRef, carId: u32, target: *mut CarState) -> bool;
    fn GetBallState(arena: ArenaRef, target: *mut BallState) -> bool;
    fn SetBallState(arena: ArenaRef, target: *mut BallState) -> bool;
    fn Step(arena: ArenaRef, ticksToSimulate: i32) -> bool;
}

// Implement Arena
impl Arena {
    pub fn new(game_mode: GameMode) -> Arena {
        let arena = unsafe { CreateArena(game_mode.into()) };
        Arena {
            _ref: arena,
            car_ids: Vec::new(),
        }
    }

    pub fn add_car(&mut self, team: Team) -> u32 {
        match unsafe { AddCar(self._ref, team.into()) } {
            0 => panic!("Failed to add car"),
            car_id => {
                self.car_ids.push(car_id);
                car_id
            }
        }
    }

    pub fn remove_car(&mut self, car_id: u32) -> bool {
        match unsafe { RemoveCar(self._ref, car_id) } {
            true => {
                self.car_ids.retain(|&x| x != car_id);
                true
            }
            false => false,
        }
    }

    pub fn controls(
        &mut self,
        car_id: u32,
        throttle: f32,
        steer: f32,
        pitch: f32,
        yaw: f32,
        roll: f32,
        boost: bool,
        jump: bool,
        handbrake: bool,
    ) -> bool {
        unsafe {
            Controls(
                self._ref, car_id, throttle, steer, pitch, yaw, roll, boost, jump, handbrake,
            )
        }
    }

    pub fn get_car_state(&self, car_id: u32) -> Option<CarState> {
        unsafe {
            let mut state = MaybeUninit::uninit();
            if GetCarState(self._ref, car_id, state.as_mut_ptr()) {
                Some(state.assume_init())
            } else {
                None
            }
        }
    }

    pub fn set_car_state(&mut self, car_id: u32, mut target: CarState) -> bool {
        unsafe { SetCarState(self._ref, car_id, &mut target as *mut CarState) }
    }

    pub fn get_ball_state(&self) -> Option<BallState> {
        unsafe {
            let mut state = MaybeUninit::uninit();
            if GetBallState(self._ref, state.as_mut_ptr()) {
                Some(state.assume_init())
            } else {
                None
            }
        }
    }

    pub fn set_ball_state(&mut self, mut target: BallState) -> bool {
        unsafe { SetBallState(self._ref, &mut target as *mut BallState) }
    }

    pub fn get_game_state(&self) -> GameState {
        let car_states = self
            .car_ids
            .iter()
            .map(|id| self.get_car_state(*id).unwrap())
            .collect();

        let ball_state = self.get_ball_state().unwrap();

        GameState {
            car_states,
            ball_state,
        }
    }

    pub fn step(&mut self, ticks_to_simulate: i32) -> bool {
        unsafe { Step(self._ref, ticks_to_simulate) }
    }
}

// Implement Drop
impl Drop for Arena {
    fn drop(&mut self) {
        unsafe { DestroyArena(self._ref) }
    }
}

#[cfg(test)]
mod tests {
    use std::time;

    use super::*;

    #[test]
    fn test_arena() {
        let mut arena = Arena::new(GameMode::Soccar);
        let car_id = arena.add_car(Team::Blue);
        let car_state = arena.get_car_state(car_id);
        println!("Car state: {:?}", car_state);
        arena.controls(car_id, 1.0, 0.0, 0.0, 0.0, 0.0, false, false, false);

        let now = time::Instant::now();
        const STRIDE: i32 = 8;
        const STEPS: i32 = 10240;
        for _ in 0..STEPS {
            arena.step(STRIDE.into());
        }
        let elapsed = now.elapsed();
        let car_state = arena.get_car_state(car_id);
        println!("Car state: {:?}", car_state);
        let ball_state = arena.get_ball_state();
        println!("Ball state: {:?}", ball_state);
        let game_state = arena.get_game_state();
        println!("Game state: {:?}", game_state);
        println!("Total steps: {}", STEPS * STRIDE);
        println!(
            "Steps per second: {}",
            ((STEPS as f64) * (STRIDE as f64)) / elapsed.as_secs_f64()
        );

        arena.remove_car(car_id);
        arena.step(1);
    }
}
