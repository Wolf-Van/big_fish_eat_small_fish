use eframe::egui;
use crate::game::GameState;

pub struct InputHandler;

impl Default for InputHandler {
    fn default() -> Self {
        Self
    }
}

impl InputHandler {
    pub fn handle_input(&self, ctx: &egui::Context, game_state: &mut GameState) -> bool {
        // 重置输入状态
        game_state.input.move_up = false;
        game_state.input.move_down = false;
        game_state.input.move_left = false;
        game_state.input.move_right = false;

        // 检查键盘输入 - 使用持续按键检测
        let keys_down = ctx.input(|i| i.keys_down.clone());
        
        // 检查按键是否被按下
        if keys_down.contains(&egui::Key::W) || keys_down.contains(&egui::Key::ArrowUp) {
            game_state.input.move_up = true;
        }
        if keys_down.contains(&egui::Key::S) || keys_down.contains(&egui::Key::ArrowDown) {
            game_state.input.move_down = true;
        }
        if keys_down.contains(&egui::Key::A) || keys_down.contains(&egui::Key::ArrowLeft) {
            game_state.input.move_left = true;
        }
        if keys_down.contains(&egui::Key::D) || keys_down.contains(&egui::Key::ArrowRight) {
            game_state.input.move_right = true;
        }
        
        // 检查ESC键是否被按下（用于暂停游戏）
        if keys_down.contains(&egui::Key::Escape) {
            return true; // 返回true表示需要暂停游戏
        }
        
        false // 返回false表示不需要暂停
    }
}
