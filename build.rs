fn main() {
    let mut build = cc::Build::new();

    build
        .compiler("clang++")
        .cpp(true)
        .flag("-std=c++20")
        .static_flag(true)
        .use_plt(false)
        .flag("-ffast-math");

    if cfg!(feature = "fat-lto") {
        build.flag("-flto=full");
    } else if cfg!(feature = "lto") {
        build.flag("-flto=thin");
    }

    build
        .file("RocketSim/libsrc/bullet3-3.24/btBulletCollisionAll.cpp")
        .file("RocketSim/libsrc/bullet3-3.24/btBulletDynamicsAll.cpp")
        .file("RocketSim/libsrc/bullet3-3.24/btLinearMathAll.cpp")
        .file("RocketSim/src/BulletLink.cpp")
        .file("RocketSim/src/Math/Math.cpp")
        .file("RocketSim/src/Sim/Arena/Arena.cpp")
        .file("RocketSim/src/Sim/Ball/Ball.cpp")
        .file("RocketSim/src/Sim/BoostPad/BoostPad.cpp")
        .file("RocketSim/src/Sim/btVehicleRL/btVehicleRL.cpp")
        .file("RocketSim/src/Sim/Car/Car.cpp")
        .file("RocketSim/src/Sim/Car/CarConfig/CarConfig.cpp")
        .file("RocketSim/src/Sim/MeshLoader/MeshLoader.cpp")
        .include("RocketSim/src")
        .file("RocketSimC/RocketSimC.cpp")
        .compile("RocketSimC");
}
