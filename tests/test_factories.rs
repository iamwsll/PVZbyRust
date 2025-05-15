use pvz_rust_wsll_lib::plants::plant_factory::{PlantFactory, PlantType};
use pvz_rust_wsll_lib::zombies::zombie_factory::{ZombieFactory, ZombieType};

#[test]
fn test_plant_factory() {
    // 测试工厂能否创建不同类型的植物
    let peashooter = PlantFactory::create_plant(PlantType::Peashooter);
    let sunflower = PlantFactory::create_plant(PlantType::Sunflower);
    let wallnut = PlantFactory::create_plant(PlantType::WallNut);
    
    // 测试获取植物成本
    assert!(PlantType::Peashooter.cost() > 0);
    assert!(PlantType::Sunflower.cost() > 0);
    assert!(PlantType::WallNut.cost() > 0);
    
    // 测试获取植物生命值
    assert!(peashooter.get_initial_health() > 0);
    assert!(sunflower.get_initial_health() > 0);
    assert!(wallnut.get_initial_health() > 0);
    
    // 验证不同植物有不同的冷却时间
    assert!(peashooter.get_cooldown() > 0);
    assert!(sunflower.get_cooldown() > 0);
    assert!(wallnut.get_cooldown() > 0);
}

#[test]
fn test_zombie_factory() {
    // 测试工厂能否创建不同类型的僵尸
    let normal_zombie = ZombieFactory::create_zombie(ZombieType::Normal);
    let conehead_zombie = ZombieFactory::create_zombie(ZombieType::Conehead);
    
    // 测试僵尸初始健康值
    assert!(normal_zombie.get_initial_health() > 0);
    assert!(conehead_zombie.get_initial_health() > 0);
    
    // 路障僵尸应该比普通僵尸有更多的生命值
    assert!(conehead_zombie.get_initial_health() > normal_zombie.get_initial_health());
    
    // 测试僵尸攻击属性
    assert!(normal_zombie.get_attack_damage() > 0);
    assert!(conehead_zombie.get_attack_damage() > 0);
    assert!(normal_zombie.get_attack_interval() > 0);
    assert!(conehead_zombie.get_attack_interval() > 0);
}

#[test]
fn test_plant_type_enum() {
    // 测试植物类型枚举的相等性
    assert_eq!(PlantType::Peashooter, PlantType::Peashooter);
    assert_ne!(PlantType::Peashooter, PlantType::Sunflower);
    assert_ne!(PlantType::Sunflower, PlantType::WallNut);
}

#[test]
fn test_zombie_type_enum() {
    // 测试僵尸类型枚举的相等性
    assert_eq!(ZombieType::Normal, ZombieType::Normal);
    assert_ne!(ZombieType::Normal, ZombieType::Conehead);
}
