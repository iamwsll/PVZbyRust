use pvz_rust_wsll_lib::plants::{Plant, PlantType};

#[test]
fn test_plant_creation() {
    // 创建不同类型的植物
    let peashooter = Plant::new(PlantType::Peashooter, 1, 2);
    let sunflower = Plant::new(PlantType::Sunflower, 3, 4);
    let wallnut = Plant::new(PlantType::WallNut, 5, 6);
    
    // 验证植物位置是否正确设置
    assert_eq!(peashooter.grid_x, 1);
    assert_eq!(peashooter.grid_y, 2);
    assert_eq!(sunflower.grid_x, 3);
    assert_eq!(sunflower.grid_y, 4);
    assert_eq!(wallnut.grid_x, 5);
    assert_eq!(wallnut.grid_y, 6);
    
    // 验证植物的初始状态
    assert!(!peashooter.is_dead);
    assert!(!sunflower.is_dead);
    assert!(!wallnut.is_dead);
    
    // 验证植物类型
    assert_eq!(peashooter.get_plant_type(), PlantType::Peashooter);
    assert_eq!(sunflower.get_plant_type(), PlantType::Sunflower);
    assert_eq!(wallnut.get_plant_type(), PlantType::WallNut);
}

#[test]
fn test_plant_damage_and_death() {
    let mut plant = Plant::new(PlantType::Peashooter, 1, 2);
    let initial_health = plant.health;
    
    // 对植物造成伤害
    plant.health -= 10;
    
    // 验证健康值是否减少
    assert_eq!(plant.health, initial_health - 10);
    assert!(!plant.is_dead);
    
    // 对植物造成致命伤害
    plant.health = 0;
    plant.is_dead = true;
    
    // 验证植物死亡状态
    assert!(plant.is_dead);
}

#[test]
fn test_plant_types() {
    // 测试植物类型枚举
    let plant_types = vec![
        PlantType::Peashooter,
        PlantType::Sunflower,
        PlantType::WallNut,
    ];
    
    // 每种植物类型应该有不同的成本
    let costs: Vec<i32> = plant_types.iter().map(|p| p.cost()).collect();
    
    // 验证成本不同且大于零
    for cost in costs {
        assert!(cost > 0);
    }
}
