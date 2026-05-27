// ui/question_panel.rs
// 目的: 機能3「設問ごとの選択肢数設定 + 完了チェックボックス」パネルを描画する。
// 全設問のリストを表形式で表示し、各行で選択肢数を変更できるようにする。
// 末尾に「設定完了」チェックボックスを設置する。
// 問題本文は250文字以内は全文表示、250文字超の場合のみ省略する。

use egui::Ui;

use crate::app::AppState;

/// 問題本文の最大表示文字数 (これを超える場合のみ省略する)
const MAX_BODY_CHARS: usize = 250;

/// 選択肢数の選択肢リスト (2〜8)
const CHOICE_OPTIONS: &[u32] = &[2, 3, 4, 5, 6, 7, 8];

/// 設問設定パネルを描画する
pub fn show(ui: &mut Ui, state: &mut AppState) {
    ui.heading("選択肢数の設定");
    ui.add_space(8.0);

    let sheet = match state.question_sheet.as_mut() {
        Some(s) => s,
        None => {
            ui.label("先に設問ファイルを取り込んでください。");
            return;
        }
    };

    // テーブルヘッダー
    egui::Grid::new("question_grid")
        .num_columns(4)
        .spacing([12.0, 6.0])
        .striped(true)
        .show(ui, |ui| {
            ui.strong("No");
            ui.strong("問題番号");
            ui.strong("問題本文");
            ui.strong("選択肢数");
            ui.end_row();

            for q in sheet.questions.iter_mut() {
                // No
                ui.label(q.no.to_string());
                // 問題番号
                ui.label(&q.question_no);
                // 問題本文: 250文字以内は全文表示、超える場合のみ省略する
                let char_count = q.body.chars().count();
                let display_body = if char_count > MAX_BODY_CHARS {
                    let truncated: String = q.body.chars().take(MAX_BODY_CHARS).collect();
                    format!("{}…", truncated)
                } else {
                    q.body.clone()
                };
                ui.label(display_body);
                // 選択肢数 ComboBox
                egui::ComboBox::from_id_source(format!("choice_count_{}", q.no))
                    .selected_text(q.num_choices.to_string())
                    .show_ui(ui, |ui| {
                        for &n in CHOICE_OPTIONS {
                            ui.selectable_value(&mut q.num_choices, n, n.to_string());
                        }
                    });
                ui.end_row();
            }
        });

    ui.add_space(12.0);

    // 設定完了チェックボックス
    ui.checkbox(
        &mut state.settings_confirmed,
        "設定が完了しました（全設問の選択肢数を確認済み）",
    );
}
