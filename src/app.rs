use eframe::egui;
use egui_chinese_font::setup_chinese_fonts;
use crate::game::GameState;
use crate::ui::UI;
use crate::input::InputHandler;
use crate::database::GameDatabase;

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
        }
    }
}

impl eframe::App for BigFishApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.current_state {
            AppState::Home => {
                self.ui.show_home_page(ctx, &mut self.current_state);
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
                self.ui.show_pause_page(ctx, &self.game_state, &mut self.current_state);
            }
            AppState::GameOver => {
                self.ui.show_game_over_page(ctx, &self.game_state, &mut self.current_state, &mut self.needs_reset);
            }
        }
    }
}

impl BigFishApp {
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
            screen_size.y
        );

        // 更新所有敌人
        for enemy in &mut self.game_state.enemies {
            enemy.update(delta_time);
        }
    }

    // 检查碰撞
    fn check_collisions(&mut self) {
        let player_pos = self.game_state.player_fish.position;
        let player_size = self.game_state.player_fish.size * 30.0;
        let player_fish_size = self.game_state.player_fish.size;

        // 检查与每个敌人的碰撞
        for enemy in &mut self.game_state.enemies {
            if enemy.check_collision_with_player(player_pos, player_size) {
                // 检查玩家是否可以吃掉这个敌人
                if player_fish_size > enemy.size_type.get_size() {
                    // 玩家吃掉敌人
                    self.game_state.score += enemy.size_type.get_score();
                    // 使用对数增长：成长速度随等级递减，增加可玩性
                    let base_growth = 0.15; // 基础成长值
                    let size_factor = self.game_state.player_fish.size;
                    let growth_rate = base_growth / (1.0 + size_factor * 0.3); // 对数衰减
                    self.game_state.player_fish.size += growth_rate;
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
