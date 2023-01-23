use std::env;

fn main() {
    let mut build = cc::Build::new();

    build
        .compiler("clang++")
        .cpp(true)
        .flag("-std=c++20")
        .static_flag(true)
        .use_plt(false);

    match env::var("PROFILE").unwrap().as_str() {
        "release" => {
            build.flag("-flto=thin");
        }
        _ => (),
    }

    build
        .define("BT_USE_SSE", None)
        .define("BT_USE_SSE_IN_API", None)
        .define("BT_USE_SIMD_VECTOR3", None)
        .define("BT_THREADSAFE", Some("1"))
        .flag("--include=xmmintrin.h")
        .file("RocketSim/libsrc/bullet3-3.24/btBulletCollisionAll.cpp")
        .file("RocketSim/libsrc/bullet3-3.24/btBulletDynamicsAll.cpp")
        .file("RocketSim/libsrc/bullet3-3.24/btLinearMathAll.cpp")
        .file("RocketSim/src/Math/Math.cpp")
        .file("RocketSim/src/Sim/Arena/Arena.cpp")
        .file("RocketSim/src/Sim/Ball/Ball.cpp")
        .file("RocketSim/src/Sim/btVehicleRL/btVehicleRL.cpp")
        .file("RocketSim/src/Sim/Car/Car.cpp")
        .file("RocketSim/src/Sim/Car/CarConfig/CarConfig.cpp")
        .file("RocketSim/src/Sim/MeshLoader/MeshLoader.cpp")
        .include("RocketSim/src")
        .file("RocketSimC/RocketSimC.cpp")
        .compile("RocketSimC");
}
