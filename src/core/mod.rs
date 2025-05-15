//! # 核心模块 (`core`)
//!
//! 本模块是植物大战僵尸游戏的核心引擎，封装了游戏运行的基础框架和主要逻辑。
//! 它负责管理游戏的整体流程，包括：
//!
//! - **游戏主循环与逻辑 (`game`)**: 包含游戏的核心状态 (`GameState` 结构体) 和主要的 `EventHandler` 实现，负责处理用户输入、更新游戏世界、执行游戏规则等。
//! - **资源加载与管理 (`resources`)**: 定义 `Resources` 结构体，处理所有游戏资源（如图像、字体等）的加载和访问。
//! - **渲染逻辑 (`renderer`)**: 定义 `Renderer` 结构体，负责将游戏世界中的所有元素绘制到屏幕上。
//! - **游戏阶段定义 (`states`)**: 定义 `GameState` 枚举，用于表示和切换不同的游戏阶段，如主菜单 (`Menu`)、游戏进行中 (`InGame`) 和游戏结束 (`GameOver`)。
//!
//! 各个子模块协同工作，构成了游戏运行的基础。

/// 游戏主逻辑模块 (`game::GameState`)，包含游戏的核心状态和 `EventHandler` 实现。
pub mod game;
/// 资源管理模块 (`resources::Resources`)，负责加载和存储所有游戏资源。
pub mod resources;
/// 渲染模块 (`renderer::Renderer`)，负责将游戏场景绘制到屏幕。
pub mod renderer;
/// 游戏阶段定义模块 (`states::GameState`)，定义了如 `Menu`, `InGame`, `GameOver` 等游戏阶段。
pub mod states;