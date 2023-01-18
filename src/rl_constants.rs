use vector3::Vector3;

pub struct Vec3(f64, f64, f64);

pub const SIDE_WALL_X: f64 = 4096.0;
pub const BACK_WALL_Y: f64 = 5120.0;
pub const SIDE_WALL_Z: f64 = 2044.0;
pub const CEILING_Z: f64 = 2044.0;
pub const BACK_NET_Y: f64 = 6000.0;

pub const GOAL_HEIGHT: f64 = 642.775;

pub const ORANGE_GOAL_CENTER: Vec3 = Vec3(0.0, BACK_WALL_Y, GOAL_HEIGHT / 2.0);
pub const BLUE_GOAL_CENTER: Vec3 = Vec3(0.0, -BACK_WALL_Y, GOAL_HEIGHT / 2.0);

pub const ORANGE_GOAL_BACK: Vec3 = Vec3(0.0, BACK_NET_Y, GOAL_HEIGHT / 2.0);
pub const BLUE_GOAL_BACK: Vec3 = Vec3(0.0, -BACK_NET_Y, GOAL_HEIGHT / 2.0);

pub const BALL_RADIUS: f64 = 92.75;

pub const BALL_MAX_SPEED: f64 = 6000.0;
pub const CAR_MAX_SPEED: f64 = 2300.0;
pub const SUPERSONIC_THRESHOLD: f64 = 2200.0;
pub const CAR_MAX_ANG_VEL: f64 = 5.5;

pub const NUM_ACTIONS: usize = 8;

pub const BOOST_LOCATIONS: [Vec3; 34] = [
    Vec3(0.0, -4240.0, 70.0),
    Vec3(-1792.0, -4184.0, 70.0),
    Vec3(1792.0, -4184.0, 70.0),
    Vec3(-3072.0, -4096.0, 73.0),
    Vec3(3072.0, -4096.0, 73.0),
    Vec3(- 940.0, -3308.0, 70.0),
    Vec3(940.0, -3308.0, 70.0),
    Vec3(0.0, -2816.0, 70.0),
    Vec3(-3584.0, -2484.0, 70.0),
    Vec3(3584.0, -2484.0, 70.0),
    Vec3(-1788.0, -2300.0, 70.0),
    Vec3(1788.0, -2300.0, 70.0),
    Vec3(-2048.0, -1036.0, 70.0),
    Vec3(0.0, -1024.0, 70.0),
    Vec3(2048.0, -1036.0, 70.0),
    Vec3(-3584.0, 0.0, 73.0),
    Vec3(-1024.0, 0.0, 70.0),
    Vec3(1024.0, 0.0, 70.0),
    Vec3(3584.0, 0.0, 73.0),
    Vec3(-2048.0, 1036.0, 70.0),
    Vec3(0.0, 1024.0, 70.0),
    Vec3(2048.0, 1036.0, 70.0),
    Vec3(-1788.0, 2300.0, 70.0),
    Vec3(1788.0, 2300.0, 70.0),
    Vec3(-3584.0, 2484.0, 70.0),
    Vec3(3584.0, 2484.0, 70.0),
    Vec3(0.0, 2816.0, 70.0),
    Vec3(- 940.0, 3310.0, 70.0),
    Vec3(940.0, 3308.0, 70.0),
    Vec3(-3072.0, 4096.0, 73.0),
    Vec3(3072.0, 4096.0, 73.0),
    Vec3(-1792.0, 4184.0, 70.0),
    Vec3(1792.0, 4184.0, 70.0),
    Vec3(0.0, 4240.0, 70.0),
];

impl Into<Vector3> for Vec3 {
    fn into(self) -> Vector3 {
        Vector3::new(self.0, self.1, self.2)
    }
}

impl Vec3 {
    pub fn to_vector3(self) -> Vector3 {
        self.into()
    }
}