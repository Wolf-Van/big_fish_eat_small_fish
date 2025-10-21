use eframe::egui;
use crate::game::PlayerFish;
use crate::enemy::{EnemyFish, EnemySize};

pub struct Renderer;

impl Default for Renderer {
    fn default() -> Self {
        Self
    }
}

impl Renderer {
    pub fn draw_player_fish(&self, ui: &mut egui::Ui, fish: &PlayerFish) {
        let fish_size = fish.size * 30.0; // 增大显示尺寸
        
        // 绘制鱼的身体（圆形）
        let fish_pos = egui::Pos2::new(fish.position.x, fish.position.y);
        let fish_color = egui::Color32::from_rgb(0, 150, 255); // 蓝色鱼
        
        // 绘制鱼的身体
        ui.painter().circle_filled(fish_pos, fish_size, fish_color);
        
        // 绘制鱼的眼睛（根据朝向调整位置）
        let eye_offset = fish_size * 0.3;
        let eye_size = fish_size * 0.2;
        let (left_eye_x, right_eye_x) = if fish.facing_right {
            // 向右游，眼睛在身体前部
            (fish_pos.x - eye_offset, fish_pos.x + eye_offset)
        } else {
            // 向左游，眼睛在身体前部
            (fish_pos.x + eye_offset, fish_pos.x - eye_offset)
        };
        
        ui.painter().circle_filled(
            egui::Pos2::new(left_eye_x, fish_pos.y - eye_offset), 
            eye_size, 
            egui::Color32::WHITE
        );
        ui.painter().circle_filled(
            egui::Pos2::new(right_eye_x, fish_pos.y - eye_offset), 
            eye_size, 
            egui::Color32::WHITE
        );
        
        // 绘制鱼的眼睛瞳孔
        let pupil_size = eye_size * 0.5;
        ui.painter().circle_filled(
            egui::Pos2::new(left_eye_x, fish_pos.y - eye_offset), 
            pupil_size, 
            egui::Color32::BLACK
        );
        ui.painter().circle_filled(
            egui::Pos2::new(right_eye_x, fish_pos.y - eye_offset), 
            pupil_size, 
            egui::Color32::BLACK
        );
        
        // 绘制鱼的尾巴（根据朝向调整位置）
        let tail_points = if fish.facing_right {
            // 向右游，尾巴在左边
            vec![
                egui::Pos2::new(fish_pos.x - fish_size, fish_pos.y),
                egui::Pos2::new(fish_pos.x - fish_size * 1.5, fish_pos.y - fish_size * 0.5),
                egui::Pos2::new(fish_pos.x - fish_size * 1.5, fish_pos.y + fish_size * 0.5),
            ]
        } else {
            // 向左游，尾巴在右边
            vec![
                egui::Pos2::new(fish_pos.x + fish_size, fish_pos.y),
                egui::Pos2::new(fish_pos.x + fish_size * 1.5, fish_pos.y - fish_size * 0.5),
                egui::Pos2::new(fish_pos.x + fish_size * 1.5, fish_pos.y + fish_size * 0.5),
            ]
        };
        ui.painter().add(egui::Shape::closed_line(tail_points, egui::Stroke::new(3.0, fish_color)));
    }

    // 绘制敌人鱼
    pub fn draw_enemy_fish(&self, ui: &mut egui::Ui, enemy: &EnemyFish) {
        if !enemy.is_alive {
            return;
        }

        let (enemy_pos, enemy_size) = enemy.get_bounds();
        let fish_pos = egui::Pos2::new(enemy_pos.x, enemy_pos.y);
        
        // 根据敌人大小选择颜色
        let fish_color = match enemy.size_type {
            EnemySize::Tiny => egui::Color32::from_rgb(255, 100, 100),      // 红色
            EnemySize::Small => egui::Color32::from_rgb(255, 150, 100),     // 橙红色
            EnemySize::Medium => egui::Color32::from_rgb(255, 200, 100),    // 橙色
            EnemySize::Large => egui::Color32::from_rgb(255, 255, 100),    // 黄色
            EnemySize::Huge => egui::Color32::from_rgb(200, 200, 200),     // 灰色
            EnemySize::Giant => egui::Color32::from_rgb(100, 255, 100),     // 绿色
            EnemySize::Massive => egui::Color32::from_rgb(100, 200, 255),   // 蓝色
            EnemySize::Colossal => egui::Color32::from_rgb(200, 100, 255),  // 紫色
            EnemySize::Titanic => egui::Color32::from_rgb(255, 100, 255),   // 粉色
            EnemySize::Legendary => egui::Color32::from_rgb(255, 255, 0),   // 金色
        };
        
        // 绘制敌人鱼的身体
        ui.painter().circle_filled(fish_pos, enemy_size, fish_color);
        
        // 绘制敌人鱼的眼睛（根据方向调整位置）
        let eye_offset = enemy_size * 0.3;
        let eye_size = enemy_size * 0.2;
        let (left_eye_x, right_eye_x) = if enemy.direction == crate::enemy::EnemyDirection::LeftToRight {
            // 向右游，眼睛在身体前部
            (fish_pos.x - eye_offset, fish_pos.x + eye_offset)
        } else {
            // 向左游，眼睛在身体前部
            (fish_pos.x + eye_offset, fish_pos.x - eye_offset)
        };
        
        ui.painter().circle_filled(
            egui::Pos2::new(left_eye_x, fish_pos.y - eye_offset), 
            eye_size, 
            egui::Color32::WHITE
        );
        ui.painter().circle_filled(
            egui::Pos2::new(right_eye_x, fish_pos.y - eye_offset), 
            eye_size, 
            egui::Color32::WHITE
        );
        
        // 绘制敌人鱼的眼睛瞳孔
        let pupil_size = eye_size * 0.5;
        ui.painter().circle_filled(
            egui::Pos2::new(left_eye_x, fish_pos.y - eye_offset), 
            pupil_size, 
            egui::Color32::BLACK
        );
        ui.painter().circle_filled(
            egui::Pos2::new(right_eye_x, fish_pos.y - eye_offset), 
            pupil_size, 
            egui::Color32::BLACK
        );
        
        // 绘制敌人鱼的尾巴（根据方向调整）
        let tail_points = if enemy.direction == crate::enemy::EnemyDirection::LeftToRight {
            // 向右游，尾巴在左边
            vec![
                egui::Pos2::new(fish_pos.x - enemy_size, fish_pos.y),
                egui::Pos2::new(fish_pos.x - enemy_size * 1.5, fish_pos.y - enemy_size * 0.5),
                egui::Pos2::new(fish_pos.x - enemy_size * 1.5, fish_pos.y + enemy_size * 0.5),
            ]
        } else {
            // 向左游，尾巴在右边
            vec![
                egui::Pos2::new(fish_pos.x + enemy_size, fish_pos.y),
                egui::Pos2::new(fish_pos.x + enemy_size * 1.5, fish_pos.y - enemy_size * 0.5),
                egui::Pos2::new(fish_pos.x + enemy_size * 1.5, fish_pos.y + enemy_size * 0.5),
            ]
        };
        ui.painter().add(egui::Shape::closed_line(tail_points, egui::Stroke::new(3.0, fish_color)));
    }
}
