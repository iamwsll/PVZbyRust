use pvz_rust_wsll_lib::zombies::{Zombie, ZombieType};

#[test]
fn test_zombie_creation() {
    // 测试不同类型僵尸的创建
    let normal_zombie = Zombie::new(ZombieType::Normal, 2);
    let cone_zombie = Zombie::new(ZombieType::Conehead, 3);
    
    // 验证僵尸的行位置
    assert_eq!(normal_zombie.row, 2);
    assert_eq!(cone_zombie.row, 3);
    
    // 验证僵尸类型
    assert_eq!(normal_zombie.get_zombie_type(), ZombieType::Normal);
    assert_eq!(cone_zombie.get_zombie_type(), ZombieType::Conehead);
    
    // 验证僵尸初始状态
    assert!(!normal_zombie.is_dying);
    assert!(!normal_zombie.death_animation_complete);
    assert!(!cone_zombie.is_dying);
    assert!(!cone_zombie.death_animation_complete);
}

#[test]
fn test_zombie_take_damage() {
    let mut zombie = Zombie::new(ZombieType::Normal, 1);
    // 僵尸没有get_health方法，我们只能检查伤害后的状态
    
    // 对僵尸造成伤害（非致命）
    let damage = 10;
    let is_dead = zombie.take_damage(damage);
    
    // 验证僵尸状态
    assert!(!is_dead);
    assert!(!zombie.is_dying);
    
    // 对僵尸造成大量伤害（可能致命）
    let fatal_damage = 100;  // 假设这个伤害足够大
    let is_dead = zombie.take_damage(fatal_damage);
    
    // 如果伤害导致死亡，验证死亡状态
    if is_dead {
        assert!(zombie.is_dying);
    }
}

#[test]
fn test_zombie_plant_detection() {
    let zombie = Zombie::new(ZombieType::Normal, 2);
    
    // 测试同行不同列的植物检测
    // 由于僵尸默认生成在x=950.0附近，我们应该调整这些值以匹配实际逻辑
    
    // 不在同一行，应该返回false
    assert!(!zombie.has_plant_in_front(1, 3));
    
    // 在同一行但距离太远，应该返回false
    assert!(!zombie.has_plant_in_front(0, 2));
}

#[test]
fn test_zombie_types() {
    // 测试僵尸类型枚举
    let zombie_types = vec![
        ZombieType::Normal,
        ZombieType::Conehead,
    ];
    
    // 创建不同类型的僵尸
    let zombies: Vec<Zombie> = zombie_types
        .iter()
        .enumerate()
        .map(|(i, &t)| Zombie::new(t, i))
        .collect();
    
    // 验证僵尸类型
    for (i, zombie) in zombies.iter().enumerate() {
        assert_eq!(zombie.get_zombie_type(), zombie_types[i]);
    }
}
