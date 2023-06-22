pub mod emulator;
pub mod player;
pub mod helpers;

#[cfg(target_os = "emscripten")]
pub mod emscripten;