use pvz_rust_wsll_lib::ui::grid::Grid;
use pvz_rust_wsll_lib::ui::shop::Shop;
use pvz_rust_wsll_lib::ui::shovel::Shovel;

#[test]
fn test_grid_creation() {
    // 创建游戏网格
    let _grid = Grid::new();
    
    // 因为我们无法直接访问Grid的属性，只是验证创建不会崩溃
    assert!(true);
}

#[test]
fn test_shop_creation() {
    // 创建商店
    let _shop = Shop::new();
    
    // 因为我们无法直接访问Shop的属性，只是验证创建不会崩溃
    assert!(true);
}

#[test]
fn test_shovel_creation() {
    // 创建铲子
    let _shovel = Shovel::new();
    
    // 因为我们无法直接访问Shovel的属性，只是验证创建不会崩溃
    assert!(true);
}
