use eframe::egui::Vec2;
use crate::enemy::{EnemyFish, EnemySpawner};
use serde::{Serialize, Deserialize};

// Vec2 serialize helpers
pub mod vec2_serde {
    use super::*;
    pub fn serialize<S>(v: &eframe::egui::Vec2, s: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        (v.x, v.y).serialize(s)
    }
    pub fn deserialize<'de, D>(d: D) -> Result<eframe::egui::Vec2, D::Error> where D: serde::Deserializer<'de> {
        let (x, y) = <(f32, f32) as serde::Deserialize>::deserialize(d)?;
        Ok(eframe::egui::Vec2 { x, y })
    }
}

// 玩家鱼类
#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerFish {
    #[serde(with = "vec2_serde")]
    pub position: Vec2,        // 位置
    #[serde(with = "vec2_serde")]
    pub velocity: Vec2,        // 速度
    pub size: f32,             // 大小
    pub health: i32,           // 血量
    pub speed: f32,            // 移动速度
    pub collision_cooldown: f32,   // 碰撞冷却时间
    pub facing_right: bool,    // 朝向：true=向右，false=向左
}

impl PlayerFish {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::ZERO,
            size: 0.25,  // 玩家初始大小，可以吃Tiny级别的鱼
            health: 100,
            speed: 300.0,       // 移动速度
            collision_cooldown: 0.0,  // 初始冷却时间为0
            facing_right: true,  // 初始朝向向右
        }
    }

    // 更新玩家鱼的位置
    pub fn update(&mut self, delta_time: f32, input: &PlayerInput) {
        // 更新碰撞冷却时间
        if self.collision_cooldown > 0.0 {
            self.collision_cooldown -= delta_time;
        }
        
        // 计算移动向量
        let mut move_vector = Vec2::ZERO;
        
        if input.move_up {
            move_vector.y -= 1.0;
        }
        if input.move_down {
            move_vector.y += 1.0;
        }
        if input.move_left {
            move_vector.x -= 1.0;
        }
        if input.move_right {
            move_vector.x += 1.0;
        }

        // 标准化移动向量（对角线移动时保持相同速度）
        if move_vector.length() > 0.0 {
            move_vector = move_vector.normalized();
        }

        // 应用速度
        self.velocity = move_vector * self.speed;

        // 根据水平移动方向更新朝向
        if self.velocity.x > 0.0 {
            self.facing_right = true;  // 向右移动
        } else if self.velocity.x < 0.0 {
            self.facing_right = false; // 向左移动
        }
        // 如果velocity.x == 0.0，保持当前朝向不变

        // 更新位置
        self.position += self.velocity * delta_time;
    }


}

// 玩家输入结构
#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerInput {
    pub move_up: bool,
    pub move_down: bool,
    pub move_left: bool,
    pub move_right: bool,
}

impl Default for PlayerInput {
    fn default() -> Self {
        Self {
            move_up: false,
            move_down: false,
            move_left: false,
            move_right: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub health: i32,
    pub size: f32,
    pub score: i32,
    pub player_fish: PlayerFish,
    pub input: PlayerInput,
    pub enemies: Vec<EnemyFish>,
    pub enemy_spawner: EnemySpawner,
    pub is_victory: bool, // 是否胜利（成为霸主）
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            health: 100,
            size: 0.25,  // 游戏状态初始大小
            score: 0,
            player_fish: PlayerFish::new(400.0, 300.0), // 初始位置在屏幕中央
            input: PlayerInput::default(),
            enemies: Vec::new(),
            enemy_spawner: EnemySpawner::default(),
            is_victory: false, // 初始为失败状态
        }
    }
}


