// ui/import_panel.rs
// 目的: 機能2「Excelファイル取り込み」パネルを描画する。
// ファイル選択ダイアログを起動し、読み込み結果をアプリ状態に格納する。

use egui::Ui;

use crate::app::AppState;
use crate::excel::reader;

/// Excel取り込みパネルを描画する
pub fn show(ui: &mut Ui, state: &mut AppState) {
    ui.heading("設問ファイルの取り込み");
    ui.add_space(8.0);

    // ファイル選択ボタン
    if ui.button("📂 Excelファイルを選択 (.xlsx)").clicked() {
        // rfd (Rusty File Dialog) を使ってファイルダイアログを開く
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Excel", &["xlsx"])
            .pick_file()
        {
            match reader::read_xlsx(&path) {
                Ok(sheet) => {
                    state.import_message = Some(format!(
                        "✓ 取り込み完了: 「{}」 {} 問",
                        sheet.subject,
                        sheet.questions.len()
                    ));
                    state.question_sheet = Some(sheet);
                }
                Err(e) => {
                    state.import_message = Some(format!("✗ 読み込みエラー: {}", e));
                    state.question_sheet = None;
                }
            }
        }
    }

    // 取り込み結果の表示
    if let Some(msg) = &state.import_message {
        ui.add_space(4.0);
        ui.label(msg.clone());
    }

    // 取り込み済みの場合は科目名と問題数を表示する
    if let Some(sheet) = &state.question_sheet {
        ui.add_space(8.0);
        ui.group(|ui| {
            ui.label(format!("科目名: {}", sheet.subject));
            ui.label(format!("設問数: {} 問", sheet.questions.len()));
        });
    }
}
