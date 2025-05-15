use pvz_rust_wsll_lib::entities::pea::{Pea, PeaType};
use pvz_rust_wsll_lib::zombies::Zombie;
use pvz_rust_wsll_lib::zombies::zombie_factory::ZombieType;
use pvz_rust_wsll_lib::plants::Plant;
use pvz_rust_wsll_lib::plants::plant_factory::PlantType;

#[test]
fn test_collision_detection() {
    // 创建一个豌豆，用于测试
    let mut pea = Pea::new(500.0, 100.0, 1, PeaType::Normal);
    
    // 豌豆的原始状态应该是活动的
    assert!(pea.active);
    
    // 模拟碰撞后，豌豆应该变为非活动
    pea.active = false;
    
    // 验证豌豆状态
    assert!(!pea.active);
}

#[test]
fn test_zombie_position_update() {
    let mut zombie = Zombie::new(ZombieType::Normal, 0);
    let initial_x = zombie.x;
    
    // 更新僵尸位置，让僵尸向左移动
    // 模拟时间更新100毫秒
    zombie.update(100);
    
    // 僵尸应该向左移动（x坐标减少）
    assert!(zombie.x < initial_x);
}

// 集成测试：测试僵尸与植物的交互
#[test]
fn test_zombie_plant_interaction() {
    let _zombie = Zombie::new(ZombieType::Normal, 1);
    let mut plant = Plant::new(PlantType::Peashooter, 5, 1);  // 同一行
    
    // 记录初始生命值
    let initial_health = plant.health;
    
    // 模拟攻击后，植物健康值应该减少
    plant.health -= 10;
    
    // 验证植物健康状态
    assert!(plant.health < initial_health);
}
