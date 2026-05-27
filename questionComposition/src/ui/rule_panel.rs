// ui/rule_panel.rs
// 目的: 機能4「選択肢生成ルールの読み込み」パネルを描画する。
// デフォルトルールの表示と、カスタム .md ファイルの取り込みを提供する。

use egui::Ui;

use crate::app::AppState;
use crate::rule_loader::RuleSet;

/// ルール設定パネルを描画する
pub fn show(ui: &mut Ui, state: &mut AppState) {
    ui.heading("選択肢生成ルール");
    ui.add_space(8.0);

    // 現在のルールソース表示
    ui.horizontal(|ui| {
        ui.label("現在のルール:");
        ui.strong(&state.rule_set.source_label);
    });

    ui.add_space(8.0);

    // カスタムルール読み込みボタン
    if ui.button("📄 カスタムルール (.md) を読み込む").clicked() {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Markdown", &["md"])
            .pick_file()
        {
            match RuleSet::load_from_file(&path) {
                Ok(rule_set) => {
                    state.rule_message = Some(format!(
                        "✓ ルールを読み込みました: {}",
                        rule_set.source_label
                    ));
                    state.rule_set = rule_set;
                    state.current_rules_are_default = false;  // カスタムルール使用
                }
                Err(e) => {
                    state.rule_message = Some(format!("✗ 読み込みエラー: {}", e));
                }
            }
        }
    }

    // デフォルトに戻すボタン
    if ui.button("デフォルトルールに戻す").clicked() {
        state.rule_set = RuleSet::load_defaults();
        state.rule_message = Some("✓ デフォルトルールに戻しました".to_string());
        state.current_rules_are_default = true;  // デフォルトルール使用
    }

    // メッセージ表示
    if let Some(msg) = &state.rule_message {
        ui.add_space(4.0);
        ui.label(msg.clone());
    }

    ui.add_space(8.0);

    // ルール本文をプレビュー表示 (スクロール可能)
    // デフォルトルール（暗号化）の場合は表示しない（機能4の要件）
    if !state.current_rules_are_default {
        ui.label("ルール内容プレビュー:");
        egui::ScrollArea::vertical()
            .id_source("rule_preview_scroll")
            .max_height(200.0)
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut state.rule_set.content.clone())
                        .desired_width(f32::INFINITY)
                        .interactive(false),
                );
            });
    } else {
        ui.colored_label(
            egui::Color32::GRAY,
            "※ デフォルトルールは表示されません（暗号化保護）",
        );
    }
}
