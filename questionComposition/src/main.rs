// main.rs
// 目的: アプリケーションのエントリポイント。
// eframe ウィンドウを初期化して App を起動する。

// lib.rs で公開されているモジュールは use で参照する
use question_composition::{config, excel, export, generator, llm, model, rule_loader};

mod app;
mod ui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("設問作成支援ツール")
            .with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    eframe::run_native(
        "設問作成支援ツール",
        native_options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    )
}
