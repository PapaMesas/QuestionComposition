// app.rs
// 目的: アプリ全体の状態 (AppState) と egui のメイン描画ループを定義する。
// タブ形式で「API設定」「設問取り込み」「選択肢数設定」「ルール設定」「作問生成」を切り替える。

use crate::config::{self, AppConfig};
use crate::model::{QuestionSheet, QuestionWithChoices};
use crate::rule_loader::RuleSet;
use crate::ui::{export_panel, generate_panel, import_panel, question_panel, rule_panel, settings_panel};

/// アプリ全体で共有する状態
pub struct AppState {
    // --- 設定関連 ---
    pub config: AppConfig,
    /// 入力中の APIキー (平文; 保存後はクリアする)
    pub api_key_input: String,
    /// APIキーの表示/非表示フラグ
    pub show_api_key: bool,
    /// APIキーが登録済みかどうか
    pub api_key_registered: bool,
    /// 設定パネルのメッセージ
    pub settings_message: Option<String>,

    // --- 取り込み関連 ---
    /// 取り込んだ設問データ
    pub question_sheet: Option<QuestionSheet>,
    /// 取り込みパネルのメッセージ
    pub import_message: Option<String>,

    // --- 選択肢数設定関連 ---
    /// 設定完了チェックボックスの状態
    pub settings_confirmed: bool,

    // --- ルール関連 ---
    pub rule_set: RuleSet,
    /// ルールパネルのメッセージ
    pub rule_message: Option<String>,

    // --- 生成関連 ---
    /// 生成中フラグ
    pub generating: bool,
    pub generation_progress: usize,
    pub generation_total: usize,
    /// 生成された選択肢データ
    pub generated_questions: Vec<QuestionWithChoices>,
    /// 生成時の科目名 (出力用)
    pub generated_subject: String,
    /// 生成パネルのメッセージ
    pub generate_message: Option<String>,

    // --- エクスポート関連 ---
    pub export_panel: export_panel::ExportPanelState,
}

impl AppState {
    /// 起動時に設定ファイルとデフォルトルールを読み込んで初期化する
    pub fn new() -> Self {
        let config = config::load();
        // 暗号化されたキーがあれば登録済みとみなす
        let api_key_registered = config.encrypted_api_key.is_some();

        Self {
            config,
            api_key_input: String::new(),
            show_api_key: false,
            api_key_registered,
            settings_message: None,
            question_sheet: None,
            import_message: None,
            settings_confirmed: false,
            rule_set: RuleSet::default(),
            rule_message: None,
            generating: false,
            generation_progress: 0,
            generation_total: 0,
            generated_questions: Vec::new(),
            generated_subject: String::new(),
            generate_message: None,
            export_panel: export_panel::ExportPanelState::default(),
        }
    }
}

/// アクティブなタブ
#[derive(PartialEq)]
enum Tab {
    Settings,
    Import,
    Questions,
    Rules,
    Generate,
    Export,
}

/// コンパイル時に NotoSansJP-Medium.ttf をバイナリへ埋め込む
/// OFL ライセンス (https://openfontlicense.org) — 埋め込み・再配布ともに許可
static NOTO_SANS_JP: &[u8] = include_bytes!("../assets/NotoSansJP-Medium.ttf");

/// eframe アプリ本体
pub struct App {
    state: AppState,
    active_tab: Tab,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // NotoSansJP-Medium を egui のフォントとして全ファミリーに適用する
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "NotoSansJP".to_owned(),
            egui::FontData::from_static(NOTO_SANS_JP),
        );
        // Proportional (通常テキスト) の先頭に挿入して最優先にする
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "NotoSansJP".to_owned());
        // Monospace (コード等) にも追加してフォールバックとして使う
        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .push("NotoSansJP".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        Self {
            state: AppState::new(),
            active_tab: Tab::Settings,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // アプリタイトル
            ui.heading("設問作成支援ツール");
            ui.separator();

            // タブバー
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tab, Tab::Settings, "① API設定");
                ui.selectable_value(&mut self.active_tab, Tab::Import, "② 設問取り込み");
                ui.selectable_value(&mut self.active_tab, Tab::Questions, "③ 選択肢数設定");
                ui.selectable_value(&mut self.active_tab, Tab::Rules, "④ ルール設定");
                ui.selectable_value(&mut self.active_tab, Tab::Generate, "⑤ 作問生成");
                ui.selectable_value(&mut self.active_tab, Tab::Export, "⑥ エクスポート");
            });
            ui.separator();
            ui.add_space(8.0);

            // アクティブなタブのパネルを表示する
            egui::ScrollArea::vertical().show(ui, |ui| match self.active_tab {
                Tab::Settings => settings_panel::show(ui, &mut self.state),
                Tab::Import => import_panel::show(ui, &mut self.state),
                Tab::Questions => question_panel::show(ui, &mut self.state),
                Tab::Rules => rule_panel::show(ui, &mut self.state),
                Tab::Generate => generate_panel::show(ui, &mut self.state),
                Tab::Export => export_panel::show(ui, &mut self.state),
            });
        });
    }
}
