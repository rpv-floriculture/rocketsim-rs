use glob::glob;
use miette::{IntoDiagnostic, Result};

fn main() -> Result<()> {
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
        .define("BT_USE_SSE", None)
        .define("BT_USE_SSE_IN_API", None)
        .define("BT_USE_SIMD_VECTOR3", None)
        .files(
            glob("RocketSim/libsrc/**/*All.cpp")
                .into_diagnostic()?
                .flatten(),
        )
        .files(glob("RocketSim/src/**/*.cpp").into_diagnostic()?.flatten())
        .include("RocketSim/src")
        .file("RocketSimC/RocketSimC.cpp")
        .compile("RocketSimC");

    Ok(())
}
