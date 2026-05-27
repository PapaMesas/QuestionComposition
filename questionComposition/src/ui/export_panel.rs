// ui/export_panel.rs
// 目的: 機能6を実装。ユーザーがエクスポート形式を選択し、保存先を指定できるUI。
// Moodle XML、QTI 1.2、QTI 2.1、QTI 2.2、QTI 3.0 から選択可能。

use egui::Ui;
use std::path::PathBuf;
use crate::app::AppState;
use crate::export::{ExportFormat, create_exporter};

/// エクスポートパネルの状態管理
#[derive(Debug, Clone)]
pub struct ExportPanelState {
    /// 選択されたエクスポート形式
    pub selected_format: ExportFormat,
    /// 出力ファイルパス
    pub output_path: Option<PathBuf>,
    /// 出力ファイル名（ユーザーが指定）
    pub file_name: String,
    /// エクスポート完了メッセージ
    pub export_message: String,
}

impl Default for ExportPanelState {
    fn default() -> Self {
        Self {
            selected_format: ExportFormat::MoodleXml,
            output_path: None,
            file_name: String::from("output.xml"),
            export_message: String::new(),
        }
    }
}

impl ExportPanelState {
    /// エクスポートパネルのUI描画
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.heading("エクスポート設定");

        // 形式選択
        ui.label("エクスポート形式を選択:");
        let formats = [
            ExportFormat::MoodleXml,
            ExportFormat::Qti12,
            ExportFormat::Qti21,
            ExportFormat::Qti22,
            ExportFormat::Qti30,
        ];

        for format in &formats {
            if ui.radio(self.selected_format == *format, format.to_string()).clicked() {
                self.selected_format = *format;
                // ファイル拡張子に応じてファイル名を更新
                let base_name = self.file_name
                    .split('.')
                    .next()
                    .unwrap_or("output");
                self.file_name = format!("{}.{}", base_name, self.selected_format.file_extension());
            }
        }

        ui.separator();

        // ファイル名入力
        ui.label("ファイル名:");
        ui.text_edit_singleline(&mut self.file_name);

        ui.separator();

        // 出力先表示
        ui.label("出力先:");
        if let Some(path) = &self.output_path {
            ui.label(format!("📁 {}", path.display()));
        } else {
            ui.label("(未選択)");
        }

        // ファイル選択ボタン
        if ui.button("📂 出力先を選択").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("XML", &["xml"])
                .save_file()
            {
                self.output_path = Some(path);
            }
        }

        ui.separator();

        // 完了メッセージ表示
        if !self.export_message.is_empty() {
            ui.colored_label(
                egui::Color32::GREEN,
                &self.export_message,
            );
        }

        // エクスポートボタンは、呼び出し元で有効/無効を制御する
        // （設問が存在し、出力先が選択されている場合のみ有効）
    }

    /// エクスポート完了メッセージを設定
    pub fn set_success_message(&mut self, message: String) {
        self.export_message = message;
    }

    /// エクスポートエラーメッセージを設定
    pub fn set_error_message(&mut self, error: String) {
        self.export_message = format!("❌ エラー: {}", error);
    }

    /// 次のエクスポートのために状態をリセット
    pub fn reset_message(&mut self) {
        self.export_message.clear();
    }
}

/// エクスポートパネルの UI を描画し、エクスポート処理を統合する
pub fn show(ui: &mut Ui, state: &mut AppState) {
    // 生成済みの設問があるかチェック
    let has_questions = !state.generated_questions.is_empty();

    if !has_questions {
        ui.colored_label(egui::Color32::YELLOW, "⚠️ まず「作問生成」で設問を生成してください");
        return;
    }

    // エクスポートパネルの UI を描画
    state.export_panel.ui(ui);

    ui.separator();

    // エクスポートボタン
    let can_export = state.export_panel.output_path.is_some() && has_questions;

    if ui.add_enabled(can_export, egui::Button::new("📤 エクスポート").min_size(egui::vec2(150.0, 40.0))).clicked() {
        perform_export(state);
    }

    if !can_export && !state.generated_questions.is_empty() {
        ui.label("💡 出力先を選択してからエクスポートできます");
    }
}

/// エクスポート処理を実行
fn perform_export(state: &mut AppState) {
    state.export_panel.reset_message();

    if let Some(path) = &state.export_panel.output_path {
        let exporter = create_exporter(state.export_panel.selected_format);

        match exporter.export(
            &state.generated_questions,
            &state.generated_subject,
            path,
        ) {
            Ok(()) => {
                let message = format!(
                    "✅ エクスポート完了: {} (形式: {})",
                    path.display(),
                    state.export_panel.selected_format
                );
                state.export_panel.set_success_message(message);
            }
            Err(e) => {
                state.export_panel.set_error_message(e.to_string());
            }
        }
    }
}
