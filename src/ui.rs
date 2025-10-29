use eframe::egui;
use crate::app::AppState;
use crate::game::GameState;
use crate::render::Renderer;
use crate::database::GameDatabase;

pub struct UI {
    pub renderer: Renderer,
}

impl Default for UI {
    fn default() -> Self {
        Self {
            renderer: Renderer::default(),
        }
    }
}

impl UI {
    pub fn show_home_page(&mut self, ctx: &egui::Context, current_state: &mut AppState) {
        // 设置海蓝色背景
        ctx.set_visuals(egui::Visuals {
            window_fill: egui::Color32::from_rgb(0, 100, 200), // 海蓝色
            panel_fill: egui::Color32::from_rgb(0, 100, 200),  // 海蓝色
            ..Default::default()
        });

        // 创建主面板
        egui::CentralPanel::default().show(ctx, |ui| {
            // 设置垂直布局，内容居中
            ui.vertical_centered(|ui| {
                ui.add_space(50.0); // 顶部间距
                
                // 添加标题"大鱼吃小鱼"
                ui.heading(egui::RichText::new("大鱼吃小鱼")
                    .size(48.0)
                    .color(egui::Color32::WHITE)
                    .strong());
                
                ui.add_space(80.0); // 标题和按钮之间的间距
                
                // 添加四个按钮（垂直排列）
                ui.vertical_centered(|ui| {
                    // 开始游戏按钮
                    if ui.add_sized([150.0, 50.0], egui::Button::new("开始游戏")).clicked() {
                        *current_state = AppState::Game;
                    }
                    
                    ui.add_space(20.0);
                    
                    // 游戏设置按钮
                    if ui.add_sized([150.0, 50.0], egui::Button::new("游戏设置")).clicked() {
                        *current_state = AppState::Settings;
                    }
                    
                    ui.add_space(20.0);
                    
                    // 历史记录按钮
                    if ui.add_sized([150.0, 50.0], egui::Button::new("历史记录")).clicked() {
                        *current_state = AppState::History;
                    }
                    
                    ui.add_space(20.0);
                    
                    // 退出游戏按钮
                    if ui.add_sized([150.0, 50.0], egui::Button::new("退出游戏")).clicked() {
                        std::process::exit(0);
                    }
                });
            });
        });
    }

    pub fn show_game_page(&mut self, ctx: &egui::Context, game_state: &GameState, current_state: &mut AppState) {
        // 设置海蓝色背景
        ctx.set_visuals(egui::Visuals {
            window_fill: egui::Color32::from_rgb(0, 100, 200), // 海蓝色
            panel_fill: egui::Color32::from_rgb(0, 100, 200),  // 海蓝色
            ..Default::default()
        });

        // 获取窗口大小
        let available_size = ctx.available_rect().size();
        let top_height = available_size.y / 8.0; // 顶部占1/8
        let bottom_height = available_size.y / 8.0; // 底部占1/8

        // 顶部区域 - 显示血量、大小、分数
        egui::TopBottomPanel::top("game_top")
            .resizable(false)
            .exact_height(top_height)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // 血量显示
                    ui.label(egui::RichText::new(format!("血量: {}", game_state.health))
                        .size(20.0)
                        .color(egui::Color32::WHITE));
                    
                    ui.add_space(50.0);
                    
                    // 大小显示
                    ui.label(egui::RichText::new(format!("大小: {:.1}", game_state.size))
                        .size(20.0)
                        .color(egui::Color32::WHITE));
                    
                    ui.add_space(50.0);
                    
                    // 分数显示
                    ui.label(egui::RichText::new(format!("分数: {}", game_state.score))
                        .size(20.0)
                        .color(egui::Color32::WHITE));
                });
            });

        // 中间区域 - 游戏主区域
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                // 绘制玩家鱼
                self.renderer.draw_player_fish(ui, &game_state.player_fish);
                
                // 绘制所有敌人鱼
                for enemy in &game_state.enemies {
                    self.renderer.draw_enemy_fish(ui, enemy);
                }
            });

        // 底部区域 - 控制区域
        egui::TopBottomPanel::bottom("game_bottom")
            .resizable(false)
            .exact_height(bottom_height)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    
                    // 返回主菜单按钮
                    if ui.add_sized([120.0, 40.0], egui::Button::new("返回主菜单")).clicked() {
                        *current_state = AppState::Home;
                    }
                    
                    ui.add_space(20.0);
                    
                    // 暂停游戏按钮
                    if ui.add_sized([120.0, 40.0], egui::Button::new("暂停游戏")).clicked() {
                        *current_state = crate::app::AppState::GamePaused;
                    }
                });
            });
    }

    // 显示设置界面
    pub fn show_settings_page(&mut self, ctx: &egui::Context, current_state: &mut AppState) {
        // 设置海蓝色背景
        ctx.set_visuals(egui::Visuals {
            window_fill: egui::Color32::from_rgb(0, 100, 200), // 海蓝色
            panel_fill: egui::Color32::from_rgb(0, 100, 200),  // 海蓝色
            ..Default::default()
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                
                // 设置标题
                ui.heading(egui::RichText::new("游戏设置")
                    .size(48.0)
                    .color(egui::Color32::WHITE)
                    .strong());
                
                ui.add_space(50.0);
                
                // 游戏说明
                ui.label(egui::RichText::new("游戏说明")
                    .size(24.0)
                    .color(egui::Color32::YELLOW));
                
                ui.add_space(20.0);
                
                ui.label(egui::RichText::new("• 使用WASD或方向键控制玩家鱼移动")
                    .size(18.0)
                    .color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("• 吃掉比自己小的鱼来成长和得分（成长速度随等级递减）")
                    .size(18.0)
                    .color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("• 避免被比自己大的鱼撞到，会扣血")
                    .size(18.0)
                    .color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("• 按ESC键可以暂停游戏")
                    .size(18.0)
                    .color(egui::Color32::WHITE));
                
                ui.add_space(30.0);
                
                // 等级说明
                ui.label(egui::RichText::new("鱼类等级")
                    .size(24.0)
                    .color(egui::Color32::YELLOW));
                
                ui.add_space(20.0);
                
                ui.label(egui::RichText::new("1级-红色  2级-橙红  3级-橙色  4级-黄色  5级-灰色")
                    .size(16.0)
                    .color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("6级-绿色  7级-蓝色  8级-紫色  9级-粉色  10级-金色")
                    .size(16.0)
                    .color(egui::Color32::WHITE));
                
                ui.add_space(50.0);
                
                // 返回主菜单按钮
                if ui.add_sized([150.0, 50.0], egui::Button::new("返回主菜单")).clicked() {
                    *current_state = AppState::Home;
                }
            });
        });
    }

    // 显示历史记录界面
    pub fn show_history_page(&mut self, ctx: &egui::Context, current_state: &mut AppState, database: &mut GameDatabase) {
        // 设置海蓝色背景
        ctx.set_visuals(egui::Visuals {
            window_fill: egui::Color32::from_rgb(0, 100, 200), // 海蓝色
            panel_fill: egui::Color32::from_rgb(0, 100, 200),  // 海蓝色
            ..Default::default()
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                
                // 历史记录标题
                ui.heading(egui::RichText::new("历史记录")
                    .size(36.0)
                    .color(egui::Color32::WHITE)
                    .strong());
                
                ui.add_space(20.0);
                
                // 记录列表
                let records = database.get_records().clone(); // 克隆记录以避免借用冲突
                if records.is_empty() {
                    ui.label(egui::RichText::new("暂无游戏记录")
                        .size(20.0)
                        .color(egui::Color32::YELLOW));
                } else {
                    // 显示记录表格 - 使用垂直布局
                    for record in records.iter().rev() { // 最新的在前面
                        ui.horizontal_centered(|ui| {
                            // 分数
                            ui.label(egui::RichText::new(format!("分数: {}", record.score))
                                .size(16.0)
                                .color(egui::Color32::WHITE));
                            
                            ui.add_space(20.0);
                            
                            // 大小
                            ui.label(egui::RichText::new(format!("大小: {:.1}", record.player_size))
                                .size(16.0)
                                .color(egui::Color32::WHITE));
                            
                            ui.add_space(20.0);
                            
                            // 时间
                            ui.label(egui::RichText::new(format!("时间: {}", record.timestamp.format("%Y-%m-%d %H:%M:%S")))
                                .size(16.0)
                                .color(egui::Color32::WHITE));
                            
                            ui.add_space(20.0);
                            
                            // 删除按钮
                            if ui.small_button("删除").clicked() {
                                database.delete_record(record.id);
                                let _ = database.save(); // 忽略保存错误
                            }
                        });
                        
                        ui.add_space(15.0);
                    }
                }
                
                ui.add_space(30.0);
                
                // 返回主菜单按钮 - 使用更明确的居中方法
                ui.allocate_ui_with_layout(
                    egui::Vec2::new(ui.available_width(), 50.0),
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        if ui.add_sized([150.0, 50.0], egui::Button::new("返回主菜单")).clicked() {
                            *current_state = AppState::Home;
                        }
                    }
                );
            });
        });
    }

    // 显示暂停界面
    pub fn show_pause_page(&mut self, ctx: &egui::Context, game_state: &crate::game::GameState, current_state: &mut crate::app::AppState) {
        // 检查ESC键是否被按下（用于恢复游戏）
        let keys_down = ctx.input(|i| i.keys_down.clone());
        if keys_down.contains(&egui::Key::Escape) {
            *current_state = crate::app::AppState::Game;
            return;
        }
        // 设置背景色为深蓝色
        ctx.set_visuals(egui::Visuals {
            window_fill: egui::Color32::from_rgb(0, 50, 100),
            panel_fill: egui::Color32::from_rgb(0, 50, 100),
            ..Default::default()
        });

        // 显示游戏状态（半透明覆盖层）
        egui::CentralPanel::default().show(ctx, |ui| {
            // 半透明背景
            let rect = ui.max_rect();
            ui.painter().rect_filled(rect, 0.0, egui::Color32::from_rgba_premultiplied(0, 0, 0, 100));
            
            // 暂停界面内容
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                
                // 暂停标题
                ui.heading(egui::RichText::new("游戏暂停")
                    .size(48.0)
                    .color(egui::Color32::WHITE));
                
                ui.add_space(50.0);
                
                // 当前分数显示
                ui.label(egui::RichText::new(format!("当前分数: {}", game_state.score))
                    .size(24.0)
                    .color(egui::Color32::YELLOW));
                
                ui.add_space(30.0);
                
                // 继续游戏按钮
                if ui.add_sized([150.0, 50.0], egui::Button::new("继续游戏")).clicked() {
                    *current_state = crate::app::AppState::Game;
                }
                
                ui.add_space(20.0);
                
                // 返回主菜单按钮
                if ui.add_sized([150.0, 50.0], egui::Button::new("返回主菜单")).clicked() {
                    *current_state = crate::app::AppState::Home;
                }
            });
        });
    }

    // 显示游戏结束界面
    pub fn show_game_over_page(&mut self, ctx: &egui::Context, game_state: &crate::game::GameState, current_state: &mut crate::app::AppState, needs_reset: &mut bool) {
        // 设置背景色为深蓝色
        ctx.set_visuals(egui::Visuals {
            window_fill: egui::Color32::from_rgb(0, 50, 100),
            panel_fill: egui::Color32::from_rgb(0, 50, 100),
            ..Default::default()
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                
                // 游戏结束标题
                ui.heading(egui::RichText::new("游戏结束")
                    .size(48.0)
                    .color(egui::Color32::WHITE));
                
                ui.add_space(50.0);
                
                // 最终分数
                ui.label(egui::RichText::new(format!("最终分数: {}", game_state.score))
                    .size(24.0)
                    .color(egui::Color32::YELLOW));
                
                ui.add_space(30.0);
                
                // 重新开始按钮
                if ui.add_sized([150.0, 50.0], egui::Button::new("重新开始")).clicked() {
                    *needs_reset = true;
                    *current_state = crate::app::AppState::Game;
                }
                
                ui.add_space(20.0);
                
                // 返回主菜单按钮
                if ui.add_sized([150.0, 50.0], egui::Button::new("返回主菜单")).clicked() {
                    *current_state = crate::app::AppState::Home;
                }
            });
        });
    }
}
