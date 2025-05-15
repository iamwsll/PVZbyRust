use pvz_rust_wsll_lib::entities::pea::{Pea, PeaType};

#[test]
fn test_pea_creation() {
    let pea = Pea::new(100.0, 200.0, 2, PeaType::Normal);
    
    assert_eq!(pea.x, 100.0);
    assert_eq!(pea.y, 200.0);
    assert_eq!(pea.row, 2);
    assert_eq!(pea.pea_type, PeaType::Normal);
    assert!(pea.active);
    assert!(pea.damage > 0);
    assert!(pea.speed > 0.0);
}

#[test]
fn test_pea_update() {
    let mut pea = Pea::new(100.0, 200.0, 2, PeaType::Normal);
    let initial_x = pea.x;
    let speed = pea.speed;
    
    // 模拟更新，使豌豆向前飞行 (100毫秒)
    pea.update(100);
    
    // 检查豌豆是否正确移动 (x坐标应该增加)
    assert!(pea.x > initial_x);
    assert_eq!(pea.x, initial_x + speed * 100.0); // 移动距离应该等于速度*时间
}

#[test]
fn test_inactive_pea() {
    let mut pea = Pea::new(100.0, 200.0, 2, PeaType::Normal);
    
    // 设置豌豆为非活动状态
    pea.active = false;
    
    // 非活动的豌豆应该能被标识
    assert!(!pea.active);
}
