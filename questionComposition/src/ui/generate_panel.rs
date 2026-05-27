// ui/generate_panel.rs
// 目的: 機能5「作問開始・進捗・出力」パネルを描画する。
// 「作問開始」ボタンは設定完了チェックかつAPIキー登録済みの場合のみ有効にする。
// 生成完了後、出力 .xlsx の保存先を選択させる。

use crate::app::AppState;
use crate::excel::writer;
use crate::generator;
use crate::llm;
use egui::Ui;

/// 作問生成パネルを描画する
pub fn show(ui: &mut Ui, state: &mut AppState) {
    ui.heading("選択肢の生成");
    ui.add_space(8.0);

    // 「作問開始」ボタンの有効条件を表示する
    let can_generate =
        state.api_key_registered && state.settings_confirmed && state.question_sheet.is_some();

    if !state.api_key_registered {
        ui.colored_label(
            egui::Color32::RED,
            "✗ APIキーが未登録です（API設定タブで登録してください）",
        );
    }
    if !state.settings_confirmed {
        ui.colored_label(egui::Color32::RED, "✗ 設定完了チェックボックスがオフです");
    }
    if state.question_sheet.is_none() {
        ui.colored_label(egui::Color32::RED, "✗ 設問ファイルが取り込まれていません");
    }

    ui.add_space(8.0);

    // 作問開始ボタン (条件を満たした場合のみ有効)
    ui.add_enabled_ui(can_generate && !state.generating, |ui| {
        if ui.button("▶ 作問開始").clicked() {
            run_generation(state);
        }
    });

    // 生成中の進捗表示
    if state.generating {
        ui.add_space(4.0);
        ui.spinner();
        ui.label(format!(
            "生成中... {}/{}",
            state.generation_progress, state.generation_total
        ));
    }

    // 生成完了メッセージ
    if let Some(msg) = &state.generate_message {
        ui.add_space(8.0);
        ui.label(msg.clone());
    }

    // 出力ファイルの保存ボタン (生成完了後に表示)
    if !state.generated_questions.is_empty() && !state.generating {
        ui.add_space(8.0);
        if ui.button("💾 結果を Excel (.xlsx) に保存").clicked() {
            save_output(state);
        }
    }
}

/// 選択肢生成を実行する (ブロッキング処理 — 設問数が多い場合はスレッド化を検討)
fn run_generation(state: &mut AppState) {
    let sheet = match &state.question_sheet {
        Some(s) => s.clone(),
        None => return,
    };

    // APIキーを復号して取得する
    let api_key = match crate::config::load_api_key(&state.config) {
        Ok(Some(k)) => k,
        _ => {
            state.generate_message = Some("✗ APIキーの読み込みに失敗しました".to_string());
            return;
        }
    };

    let client = llm::create_client(&state.config.provider, &api_key);

    state.generating = true;
    state.generation_total = sheet.questions.len();
    state.generation_progress = 0;
    state.generated_questions.clear();

    let mut results = Vec::new();
    let mut errors = Vec::new();

    for question in &sheet.questions {
        match generator::generate_choices(
            client.as_ref(),
            &sheet.subject,
            question,
            &state.rule_set,
        ) {
            Ok(qwc) => results.push(qwc),
            Err(e) => errors.push(format!("問題{}: {}", question.question_no, e)),
        }
        state.generation_progress += 1;
    }

    state.generated_questions = results;
    state.generating = false;
    state.generated_subject = sheet.subject.clone();

    if errors.is_empty() {
        state.generate_message = Some(format!(
            "✓ 生成完了: {} 問",
            state.generated_questions.len()
        ));
    } else {
        state.generate_message = Some(format!(
            "⚠ {} 問完了、{} 問でエラー: {}",
            state.generated_questions.len(),
            errors.len(),
            errors.join("; ")
        ));
    }
}

/// 生成結果を .xlsx として保存する
fn save_output(state: &mut AppState) {
    let default_name = format!(
        "output_{}.xlsx",
        chrono::Local::now().format("%Y%m%d_%H%M%S")
    );

    if let Some(path) = rfd::FileDialog::new()
        .set_file_name(&default_name)
        .add_filter("Excel", &["xlsx"])
        .save_file()
    {
        match writer::write_xlsx(&path, &state.generated_subject, &state.generated_questions) {
            Ok(_) => {
                state.generate_message = Some(format!("✓ 保存完了: {}", path.display()));
            }
            Err(e) => {
                state.generate_message = Some(format!("✗ 保存エラー: {}", e));
            }
        }
    }
}
