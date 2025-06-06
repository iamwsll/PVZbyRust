//! # 游戏机制模块 (`mechanics`)
//!
//! 本模块负责实现游戏的核心机制和规则，这些机制驱动着游戏的动态交互和进展。
//!
//! ## 主要机制：
//! - **碰撞检测 (`collision`)**: 处理游戏中不同实体（如豌豆与僵尸、僵尸与植物）之间的碰撞及其后果。
//! - **实体管理 (`entity_manager`)**: 负责在游戏过程中动态生成实体，例如自然掉落的阳光、以及根据关卡进度生成的僵尸。
//! - **关卡控制 (`level_controller`)**: 管理游戏的关卡流程，包括僵尸的生成波次、时间线以及可能的特殊事件。(注意：`level_controller.rs` 在您的文件列表中，但其内容未提供，这里的描述是基于通用游戏设计模式的推测)。

/// 碰撞检测模块，处理实体间的碰撞逻辑。
pub mod collision;
/// 实体管理器模块，负责动态生成游戏实体。
pub mod entity_manager;
/// 关卡控制器模块，管理游戏进程和僵尸生成。
pub mod level_controller;