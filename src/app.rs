use eframe::egui;
use egui_chinese_font::setup_chinese_fonts;
use crate::game::GameState;
use crate::ui::UI;
use crate::input::InputHandler;
use crate::database::GameDatabase;
use std::fs;

fn setup_custom_fonts(ctx: &egui::Context) {
    // 使用egui-chinese-font库来设置中文字体
    setup_chinese_fonts(ctx).expect("无法加载中文字体");
}

#[derive(PartialEq)]
pub enum AppState {
    Home,
    Settings,
    History,
    Game,
    GamePaused,
    GameOver,
}

pub struct BigFishApp {
    pub current_state: AppState,
    pub game_state: GameState,
    pub ui: UI,
    pub input_handler: InputHandler,
    pub needs_reset: bool,
    pub database: GameDatabase,
    pub has_saved_game: bool, // 是否有存档
}

impl Default for BigFishApp {
    fn default() -> Self {
        Self {
            current_state: AppState::Home,
            game_state: GameState::default(),
            ui: UI::default(),
            input_handler: InputHandler::default(),
            needs_reset: false,
            database: GameDatabase::load(),
            has_saved_game: false,
        }
    }
}

impl eframe::App for BigFishApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.current_state {
            AppState::Home => {
                self.ui.show_home_page(ctx, &mut self.current_state);
                // 动态判断存档按文件存取
                self.has_saved_game = fs::metadata("game_save.json").is_ok();
                if self.ui.show_continue_game {
                    if self.has_saved_game {
                        if let Some(loaded) = Self::load_game_state() {
                            self.game_state = loaded;
                            self.current_state = AppState::Game;
                            self.ui.show_continue_game = false;
                        } else {
                            egui::Window::new("提示").show(ctx, |ui| {
                                ui.label("存档损坏，请重新开始游戏");
                                if ui.button("确定").clicked() {
                                    self.ui.show_continue_game = false;
                                }
                            });
                        }
                    } else {
                        egui::Window::new("提示").show(ctx, |ui| {
                            ui.label("当前没有存档，请点击开始游戏");
                            if ui.button("确定").clicked() {
                                self.ui.show_continue_game = false;
                            }
                        });
                    }
                }
                // 新开游戏清理存档
                if self.ui.need_new_game {
                    self.game_state = GameState::default();
                    self.has_saved_game = false;
                    let _ = fs::remove_file("game_save.json");
                    self.ui.need_new_game = false;
                }
            }
            AppState::Settings => {
                self.ui.show_settings_page(ctx, &mut self.current_state);
            }
            AppState::History => {
                self.ui.show_history_page(ctx, &mut self.current_state, &mut self.database);
            }
            AppState::Game => {
                // 如果需要重置，重置游戏状态
                if self.needs_reset {
                    self.game_state = GameState::default();
                    self.needs_reset = false;
                    let _ = fs::remove_file("game_save.json"); // 重新开始清理旧进度
                }
                
                // 处理输入，检查是否需要暂停
                if self.input_handler.handle_input(ctx, &mut self.game_state) {
                    self.current_state = AppState::GamePaused;
                } else {
                    self.update_game_state(ctx);
                }
                self.ui.show_game_page(ctx, &self.game_state, &mut self.current_state);
            }
            AppState::GamePaused => {
                // 自动保存暂停状态
                let _ = Self::save_game_state(&self.game_state);
                self.ui.show_pause_page(ctx, &self.game_state, &mut self.current_state);
            }
            AppState::GameOver => {
                // 游戏结束自动清理存档
                let _ = fs::remove_file("game_save.json");
                self.ui.show_game_over_page(ctx, &self.game_state, &mut self.current_state, &mut self.needs_reset);
            }
        }
    }
}

impl BigFishApp {
    fn save_game_state(game_state: &GameState) -> Result<(), Box<dyn std::error::Error>> {
        let s = serde_json::to_string(game_state)?;
        fs::write("game_save.json", s)?;
        Ok(())
    }

    fn load_game_state() -> Option<GameState> {
        if let Ok(s) = fs::read_to_string("game_save.json") {
            serde_json::from_str(&s).ok()
        } else {
            None
        }
    }

    // 更新游戏状态
    fn update_game_state(&mut self, ctx: &egui::Context) {
        let delta_time = ctx.input(|i| i.stable_dt);
        let screen_size = ctx.available_rect().size();
        
        // 计算游戏区域边界
        let top_height = screen_size.y / 8.0; // 顶部占1/8
        let bottom_height = screen_size.y / 8.0; // 底部占1/8
        let game_area_top = top_height;
        let game_area_bottom = screen_size.y - bottom_height;
        let game_area_left = 0.0;
        let game_area_right = screen_size.x;
        
        // 更新玩家鱼
        self.game_state.player_fish.update(
            delta_time, 
            &self.game_state.input
        );
        
        // 更新敌人
        self.update_enemies(delta_time, screen_size);
        
        // 检查碰撞
        self.check_collisions();
        
        // 检查游戏区域边界碰撞
        self.check_game_area_boundary_collision(
            game_area_top, 
            game_area_bottom, 
            game_area_left, 
            game_area_right
        );
        
        // 同步游戏状态显示
        self.game_state.health = self.game_state.player_fish.health;
        self.game_state.size = self.game_state.player_fish.size;
        
        // 请求重绘以确保流畅的动画
        ctx.request_repaint();
    }

    // 检查游戏区域边界碰撞
    fn check_game_area_boundary_collision(&mut self, top: f32, bottom: f32, left: f32, right: f32) {
        let fish = &mut self.game_state.player_fish;
        let half_size = fish.size * 30.0;
        
        // 检查左右边界
        if fish.position.x - half_size < left {
            fish.position.x = left + half_size;
            fish.velocity.x = 0.0;
        }
        if fish.position.x + half_size > right {
            fish.position.x = right - half_size;
            fish.velocity.x = 0.0;
        }
        
        // 检查上下边界（游戏区域）
        if fish.position.y - half_size < top {
            fish.position.y = top + half_size;
            fish.velocity.y = 0.0;
        }
        if fish.position.y + half_size > bottom {
            fish.position.y = bottom - half_size;
            fish.velocity.y = 0.0;
        }
    }

    // 更新敌人
    fn update_enemies(&mut self, delta_time: f32, screen_size: eframe::egui::Vec2) {
        // 更新敌人生成器
        self.game_state.enemy_spawner.update(
            delta_time,
            &mut self.game_state.enemies,
            screen_size.x,
            screen_size.y,
            self.game_state.player_fish.size // 传递玩家大小
        );

        // 更新所有敌人
        for enemy in &mut self.game_state.enemies {
            enemy.update(delta_time);
        }
        
        // 调试信息：打印敌人数量
        if self.game_state.enemies.len() == 0 {
            println!("警告：当前没有敌人鱼！玩家大小: {:.2}", self.game_state.player_fish.size);
        }
    }

    // 检查碰撞
    fn check_collisions(&mut self) {
        let player_pos = self.game_state.player_fish.position;
        let player_size = self.game_state.player_fish.size * 30.0;
        let player_fish_size = self.game_state.player_fish.size;

        // 检查霸主胜利条件：玩家成为最大鱼（超过Legendary级别）
        if player_fish_size > 1.2 { // 比Legendary(1.1)更大
            // 设置胜利标志
            self.game_state.is_victory = true;
            // 保存游戏记录
            self.database.add_record(
                self.game_state.score,
                self.game_state.player_fish.size
            );
            let _ = self.database.save(); // 忽略保存错误
            self.current_state = crate::app::AppState::GameOver; // 使用GameOver状态显示胜利
            return;
        }

        // 检查与每个敌人的碰撞
        for enemy in &mut self.game_state.enemies {
            if enemy.check_collision_with_player(player_pos, player_size) {
                // 检查玩家是否可以吃掉这个敌人
                if player_fish_size > enemy.size_type.get_size() {
                    // 玩家吃掉敌人
                    self.game_state.score += enemy.size_type.get_score();
                    // 成长改为按敌人等级：每级0.01（Tiny=0.01 … Legendary=0.10）
                    let growth = enemy.size_type.growth_increment();
                    self.game_state.player_fish.size += growth;
                    self.game_state.size = self.game_state.player_fish.size; // 同步大小
                    enemy.be_eaten();
                } else {
                    // 检查碰撞冷却时间，避免连续扣血
                    if self.game_state.player_fish.collision_cooldown <= 0.0 {
                        // 玩家被大鱼伤害，扣除50点血
                        self.game_state.player_fish.health -= 50;
                        self.game_state.health = self.game_state.player_fish.health; // 同步血量
                        self.game_state.player_fish.collision_cooldown = 1.0; // 设置1秒冷却时间
                        
                    if self.game_state.player_fish.health <= 0 {
                        // 游戏结束，保存记录并进入游戏结束界面
                        self.database.add_record(
                            self.game_state.score,
                            self.game_state.player_fish.size
                        );
                        let _ = self.database.save(); // 忽略保存错误
                        self.current_state = crate::app::AppState::GameOver;
                    }
                    }
                }
            }
        }
    }

}

pub fn run_ui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_resizable(false), // 固定窗口大小
        ..Default::default()
    };
    
    eframe::run_native(
        "大鱼吃小鱼",
        options,
        Box::new(|cc| {
            // 在应用启动时设置字体
            setup_custom_fonts(&cc.egui_ctx);
            Box::new(BigFishApp::default())
        }),
    )
}
