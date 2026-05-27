// ui/settings_panel.rs
// 目的: 機能1「API設定」パネルを描画する。
// プロバイダー選択・APIキー入力・保存・検証表示を担う。

use egui::Ui;

use crate::app::AppState;
use crate::config;
use crate::model::LlmProvider;

/// API設定パネルを描画する
pub fn show(ui: &mut Ui, state: &mut AppState) {
    ui.heading("API 設定");
    ui.add_space(8.0);

    // プロバイダー選択
    ui.horizontal(|ui| {
        ui.label("LLM プロバイダー:");
        egui::ComboBox::from_id_source("provider_combo")
            .selected_text(state.config.provider.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut state.config.provider, LlmProvider::Gemini, "Gemini");
                ui.selectable_value(
                    &mut state.config.provider,
                    LlmProvider::OpenAI,
                    "ChatGPT (OpenAI)",
                );
                ui.selectable_value(
                    &mut state.config.provider,
                    LlmProvider::Claude,
                    "Claude (Anthropic)",
                );
            });
    });

    ui.add_space(8.0);

    // APIキー入力 (パスワードフィールド)
    ui.horizontal(|ui| {
        ui.label("API キー:");
        let response = ui.add(
            egui::TextEdit::singleline(&mut state.api_key_input)
                .password(!state.show_api_key)
                .hint_text("APIキーを入力してください")
                .desired_width(400.0),
        );
        // 表示/非表示トグル
        if ui
            .small_button(if state.show_api_key {
                "隠す"
            } else {
                "表示"
            })
            .clicked()
        {
            state.show_api_key = !state.show_api_key;
        }
        let _ = response;
    });

    ui.add_space(8.0);

    // ボタン行: 保存と削除
    ui.horizontal(|ui| {
        if ui.button("APIキーを保存").clicked() {
            if state.api_key_input.trim().is_empty() {
                state.settings_message = Some("⚠ APIキーを入力してください".to_string());
            } else {
                match config::store_api_key(&mut state.config, state.api_key_input.trim()) {
                    Ok(_) => {
                        state.api_key_registered = true;
                        state.api_key_input.clear();
                        state.settings_message =
                            Some("✓ APIキーを暗号化して保存しました".to_string());
                    }
                    Err(e) => {
                        state.settings_message = Some(format!("✗ 保存エラー: {}", e));
                    }
                }
            }
        }

        // 削除ボタン: APIキー登録済みの場合のみ有効
        let delete_button = ui.button("削除");
        if delete_button.clicked() && state.api_key_registered {
            // APIキーを削除（設定から暗号化キーをクリア）
            state.config.encrypted_api_key = None;
            match config::save(&state.config) {
                Ok(_) => {
                    state.api_key_registered = false;
                    state.api_key_input.clear();
                    state.settings_message = Some("✓ APIキーを削除しました".to_string());
                }
                Err(e) => {
                    state.settings_message = Some(format!("✗ 削除エラー: {}", e));
                }
            }
        }
    });

    // 登録状態の表示
    ui.add_space(4.0);
    if state.api_key_registered {
        ui.colored_label(egui::Color32::GREEN, "✓ APIキーが登録されています");
    } else {
        ui.colored_label(egui::Color32::GRAY, "APIキーが未登録です");
    }

    // メッセージ表示
    if let Some(msg) = &state.settings_message {
        ui.add_space(4.0);
        ui.label(msg.clone());
    }
}
