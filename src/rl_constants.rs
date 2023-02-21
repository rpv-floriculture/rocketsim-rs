use glam::Vec3;

pub const SIDE_WALL_X: f32 = 4096.0;
pub const BACK_WALL_Y: f32 = 5120.0;
pub const SIDE_WALL_Z: f32 = 2044.0;
pub const CEILING_Z: f32 = 2044.0;
pub const BACK_NET_Y: f32 = 6000.0;

pub const GOAL_HEIGHT: f32 = 642.775;

pub const ORANGE_GOAL_CENTER: Vec3 = Vec3::new(0.0, BACK_WALL_Y, GOAL_HEIGHT / 2.0);
pub const BLUE_GOAL_CENTER: Vec3 = Vec3::new(0.0, -BACK_WALL_Y, GOAL_HEIGHT / 2.0);

pub const ORANGE_GOAL_BACK: Vec3 = Vec3::new(0.0, BACK_NET_Y, GOAL_HEIGHT / 2.0);
pub const BLUE_GOAL_BACK: Vec3 = Vec3::new(0.0, -BACK_NET_Y, GOAL_HEIGHT / 2.0);

pub const BALL_RADIUS: f32 = 92.75;

pub const BALL_MAX_SPEED: f32 = 6000.0;
pub const CAR_MAX_SPEED: f32 = 2300.0;
pub const SUPERSONIC_THRESHOLD: f32 = 2200.0;
pub const CAR_MAX_ANG_VEL: f32 = 5.5;

pub const NUM_ACTIONS: usize = 8;

pub const BOOST_LOCATIONS: [Vec3; 34] = [
    Vec3::new(0.0, -4240.0, 70.0),
    Vec3::new(-1792.0, -4184.0, 70.0),
    Vec3::new(1792.0, -4184.0, 70.0),
    Vec3::new(-3072.0, -4096.0, 73.0),
    Vec3::new(3072.0, -4096.0, 73.0),
    Vec3::new(-940.0, -3308.0, 70.0),
    Vec3::new(940.0, -3308.0, 70.0),
    Vec3::new(0.0, -2816.0, 70.0),
    Vec3::new(-3584.0, -2484.0, 70.0),
    Vec3::new(3584.0, -2484.0, 70.0),
    Vec3::new(-1788.0, -2300.0, 70.0),
    Vec3::new(1788.0, -2300.0, 70.0),
    Vec3::new(-2048.0, -1036.0, 70.0),
    Vec3::new(0.0, -1024.0, 70.0),
    Vec3::new(2048.0, -1036.0, 70.0),
    Vec3::new(-3584.0, 0.0, 73.0),
    Vec3::new(-1024.0, 0.0, 70.0),
    Vec3::new(1024.0, 0.0, 70.0),
    Vec3::new(3584.0, 0.0, 73.0),
    Vec3::new(-2048.0, 1036.0, 70.0),
    Vec3::new(0.0, 1024.0, 70.0),
    Vec3::new(2048.0, 1036.0, 70.0),
    Vec3::new(-1788.0, 2300.0, 70.0),
    Vec3::new(1788.0, 2300.0, 70.0),
    Vec3::new(-3584.0, 2484.0, 70.0),
    Vec3::new(3584.0, 2484.0, 70.0),
    Vec3::new(0.0, 2816.0, 70.0),
    Vec3::new(-940.0, 3310.0, 70.0),
    Vec3::new(940.0, 3308.0, 70.0),
    Vec3::new(-3072.0, 4096.0, 73.0),
    Vec3::new(3072.0, 4096.0, 73.0),
    Vec3::new(-1792.0, 4184.0, 70.0),
    Vec3::new(1792.0, 4184.0, 70.0),
    Vec3::new(0.0, 4240.0, 70.0),
];
