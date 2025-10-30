use eframe::egui::Vec2;
use serde::{Serialize, Deserialize};

// 敌人鱼的大小等级
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum EnemySize {
    Tiny,      // 1级，1分
    Small,     // 2级，2分
    Medium,    // 3级，3分
    Large,     // 4级，4分
    Huge,      // 5级，5分
    Giant,     // 6级，6分
    Massive,   // 7级，7分
    Colossal,  // 8级，8分
    Titanic,   // 9级，9分
    Legendary, // 10级，10分
}

impl EnemySize {
    pub fn get_score(&self) -> i32 {
        match self {
            EnemySize::Tiny => 1,
            EnemySize::Small => 2,
            EnemySize::Medium => 3,
            EnemySize::Large => 4,
            EnemySize::Huge => 5,
            EnemySize::Giant => 6,
            EnemySize::Massive => 7,
            EnemySize::Colossal => 8,
            EnemySize::Titanic => 9,
            EnemySize::Legendary => 10,
        }
    }

    // 返回增长增量：按等级每级0.01（Tiny=0.01 … Legendary=0.10）
    pub fn growth_increment(&self) -> f32 {
        match self {
            EnemySize::Tiny => 0.01,
            EnemySize::Small => 0.02,
            EnemySize::Medium => 0.03,
            EnemySize::Large => 0.04,
            EnemySize::Huge => 0.05,
            EnemySize::Giant => 0.06,
            EnemySize::Massive => 0.07,
            EnemySize::Colossal => 0.08,
            EnemySize::Titanic => 0.09,
            EnemySize::Legendary => 0.10,
        }
    }

    // 生成权重：越小权重越高（用于偏向生成小鱼）
    pub fn spawn_weight(&self) -> u32 {
        match self {
            EnemySize::Tiny => 10,
            EnemySize::Small => 8,
            EnemySize::Medium => 6,
            EnemySize::Large => 5,
            EnemySize::Huge => 4,
            EnemySize::Giant => 3,
            EnemySize::Massive => 2,
            EnemySize::Colossal => 2,
            EnemySize::Titanic => 1,
            EnemySize::Legendary => 1,
        }
    }

    pub fn get_size(&self) -> f32 {
        match self {
            EnemySize::Tiny => 0.2,      // 1级
            EnemySize::Small => 0.3,     // 2级
            EnemySize::Medium => 0.4,    // 3级
            EnemySize::Large => 0.5,     // 4级
            EnemySize::Huge => 0.6,      // 5级
            EnemySize::Giant => 0.7,     // 6级
            EnemySize::Massive => 0.8,   // 7级
            EnemySize::Colossal => 0.9,  // 8级
            EnemySize::Titanic => 1.0,   // 9级
            EnemySize::Legendary => 1.1, // 10级
        }
    }

    pub fn get_speed(&self) -> f32 {
        match self {
            EnemySize::Tiny => 150.0,      // 小鱼游得快
            EnemySize::Small => 140.0,
            EnemySize::Medium => 130.0,
            EnemySize::Large => 120.0,
            EnemySize::Huge => 110.0,
            EnemySize::Giant => 100.0,
            EnemySize::Massive => 90.0,
            EnemySize::Colossal => 80.0,
            EnemySize::Titanic => 70.0,
            EnemySize::Legendary => 60.0,  // 大鱼游得慢
        }
    }
}

// 敌人鱼的移动方向
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum EnemyDirection {
    LeftToRight,  // 从左边进入，向右移动
    RightToLeft,  // 从右边进入，向左移动
}

// 敌人鱼结构
#[derive(Serialize, Deserialize, Clone)]
pub struct EnemyFish {
    #[serde(with = "crate::game::vec2_serde")]
    pub position: Vec2,
    #[serde(with = "crate::game::vec2_serde")]
    pub velocity: Vec2,
    pub size_type: EnemySize,
    pub direction: EnemyDirection,
    pub is_alive: bool,
}

impl EnemyFish {
    pub fn new(size_type: EnemySize, direction: EnemyDirection, start_y: f32, screen_width: f32) -> Self {
        let speed = size_type.get_speed();
        let velocity = match direction {
            EnemyDirection::LeftToRight => Vec2::new(speed, 0.0),
            EnemyDirection::RightToLeft => Vec2::new(-speed, 0.0),
        };
        
        let start_x = match direction {
            EnemyDirection::LeftToRight => -50.0,  // 从屏幕左边外开始
            EnemyDirection::RightToLeft => screen_width + 50.0,  // 从屏幕右边外开始
        };

        Self {
            position: Vec2::new(start_x, start_y),
            velocity,
            size_type,
            direction,
            is_alive: true,
        }
    }

    // 更新敌人鱼位置
    pub fn update(&mut self, delta_time: f32) {
        if self.is_alive {
            self.position += self.velocity * delta_time;
        }
    }

    // 检查是否离开游戏区域
    pub fn is_out_of_bounds(&self, screen_width: f32) -> bool {
        match self.direction {
            EnemyDirection::LeftToRight => self.position.x > screen_width + 50.0,
            EnemyDirection::RightToLeft => self.position.x < -50.0,
        }
    }

    // 获取敌人鱼的边界框
    pub fn get_bounds(&self) -> (Vec2, f32) {
        let size = self.size_type.get_size() * 30.0; // 增大显示尺寸
        (self.position, size)
    }

    // 检查与玩家鱼的碰撞
    pub fn check_collision_with_player(&self, player_pos: Vec2, player_size: f32) -> bool {
        if !self.is_alive {
            return false;
        }

        let (enemy_pos, enemy_size) = self.get_bounds();
        let distance = (enemy_pos - player_pos).length();
        let collision_distance = (enemy_size + player_size) / 2.0;
        
        distance < collision_distance
    }

    // 被玩家吃掉
    pub fn be_eaten(&mut self) {
        self.is_alive = false;
    }
}

// 敌人生成器
#[derive(Serialize, Deserialize, Clone)]
pub struct EnemySpawner {
    pub spawn_timer: f32,
    pub spawn_interval: f32,
    pub max_enemies: usize,
}

impl Default for EnemySpawner {
    fn default() -> Self {
        Self {
            spawn_timer: 0.0,
            spawn_interval: 1.0,  // 基础：每1秒尝试生成
            max_enemies: 14,      // 提高上限，让小鱼可以更多
        }
    }
}

impl EnemySpawner {
    // 更新生成器
    pub fn update(&mut self, delta_time: f32, enemies: &mut Vec<EnemyFish>, screen_width: f32, screen_height: f32, player_size: f32) {
        self.spawn_timer += delta_time;
        
        // 清理：移除已死亡或已离开屏幕的敌人，避免占满数量上限
        enemies.retain(|enemy| enemy.is_alive && !enemy.is_out_of_bounds(screen_width));
        
        // 如果敌人数量未达到上限且到了生成时间
        if enemies.len() < self.max_enemies && self.spawn_timer >= self.spawn_interval {
            self.spawn_enemy(enemies, screen_width, screen_height, player_size);
            self.spawn_timer = 0.0;
        }
    }

    // 生成新敌人
    fn spawn_enemy(&mut self, enemies: &mut Vec<EnemyFish>, screen_width: f32, screen_height: f32, player_size: f32) {
        // 根据玩家大小调整敌人生成概率
        let mut available_sizes = Vec::new();
        
        // 总是生成一些比玩家小的鱼（可被吃掉）
        if player_size > 0.2 { available_sizes.push(EnemySize::Tiny); }
        if player_size > 0.3 { available_sizes.push(EnemySize::Small); }
        if player_size > 0.4 { available_sizes.push(EnemySize::Medium); }
        if player_size > 0.5 { available_sizes.push(EnemySize::Large); }
        if player_size > 0.6 { available_sizes.push(EnemySize::Huge); }
        if player_size > 0.7 { available_sizes.push(EnemySize::Giant); }
        if player_size > 0.8 { available_sizes.push(EnemySize::Massive); }
        if player_size > 0.9 { available_sizes.push(EnemySize::Colossal); }
        if player_size > 1.0 { available_sizes.push(EnemySize::Titanic); }
        
        // 总是生成一些比玩家大的鱼（危险）
        if player_size < 0.3 { available_sizes.push(EnemySize::Small); }
        if player_size < 0.4 { available_sizes.push(EnemySize::Medium); }
        if player_size < 0.5 { available_sizes.push(EnemySize::Large); }
        if player_size < 0.6 { available_sizes.push(EnemySize::Huge); }
        if player_size < 0.7 { available_sizes.push(EnemySize::Giant); }
        if player_size < 0.8 { available_sizes.push(EnemySize::Massive); }
        if player_size < 0.9 { available_sizes.push(EnemySize::Colossal); }
        if player_size < 1.0 { available_sizes.push(EnemySize::Titanic); }
        if player_size < 1.1 { available_sizes.push(EnemySize::Legendary); }
        
        // 如果没有可用的敌人大小，至少生成Tiny
        if available_sizes.is_empty() {
            available_sizes.push(EnemySize::Tiny);
        }
        
        // 按权重随机选择大小等级（小鱼权重更高）
        let total_weight: u32 = available_sizes.iter().map(|s| s.spawn_weight()).sum();
        let mut pick = fastrand::u32(..total_weight);
        let mut size_type = available_sizes[0];
        for s in &available_sizes {
            let w = s.spawn_weight();
            if pick < w { size_type = *s; break; }
            pick -= w;
        }
        
        // 随机选择方向
        let direction = if fastrand::bool() {
            EnemyDirection::LeftToRight
        } else {
            EnemyDirection::RightToLeft
        };
        
        // 随机选择Y位置（在游戏区域内，考虑鱼的半径，避免超出上下边界）
        let game_area_top = screen_height / 8.0;
        let game_area_bottom = screen_height * 7.0 / 8.0;
        let enemy_diameter = size_type.get_size() * 30.0;
        let enemy_radius = enemy_diameter * 0.5;
        let min_y = (game_area_top + enemy_radius).min(game_area_bottom - enemy_radius);
        let max_y = (game_area_bottom - enemy_radius).max(min_y);
        let y_range = (max_y - min_y).max(0.0);
        let start_y = min_y + fastrand::f32() * y_range;
        
        let enemy = EnemyFish::new(size_type, direction, start_y, screen_width);
        enemies.push(enemy);

        // 如果是小鱼（Tiny/Small），有较大概率额外多生一条，体现“更多更快”
        if enemies.len() < self.max_enemies {
            let extra_prob = match size_type { EnemySize::Tiny => 0.6, EnemySize::Small => 0.4, _ => 0.0 };
            if fastrand::f32() < extra_prob {
                // 再随机一个方向和Y位置
                let direction2 = if fastrand::bool() { EnemyDirection::LeftToRight } else { EnemyDirection::RightToLeft };
                let start_y2 = game_area_top + fastrand::f32() * y_range;
                let extra_enemy = EnemyFish::new(size_type, direction2, start_y2, screen_width);
                enemies.push(extra_enemy);
            }
        }
    }
}
