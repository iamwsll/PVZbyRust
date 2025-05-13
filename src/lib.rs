//! pvz_rust_wsll - 一个用 Rust 实现的植物大战僵尸风格游戏的库。
//!
//! 这个库为游戏提供了核心功能、实体、机制、
//! 植物、UI 组件和僵尸。

// 声明模块并将其公开，使其成为库 API 的一部分
pub mod core;
pub mod entities;
pub mod mechanics;
pub mod plants;
pub mod ui;
pub mod zombies;


/// 一个示例函数，表明库已正确加载。
pub fn library_greeting() {
    println!("Hello from the pvz_rust_wsll library!");
}

