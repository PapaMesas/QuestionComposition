// export/mod.rs
// 目的: QTI、Moodle XML 等の複数の出力形式に対応するエクスポート機能を提供する。
// 各形式のハンドラーは ExportHandler トレイトを実装する。

use crate::model::QuestionWithChoices;
use std::path::PathBuf;

mod moodle;
mod qti12;
mod qti21;
mod qti22;
mod qti30;

pub use moodle::MoodleExporter;
pub use qti12::Qti12Exporter;
pub use qti21::Qti21Exporter;
pub use qti22::Qti22Exporter;
pub use qti30::Qti30Exporter;

/// エクスポート形式の選択肢
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    MoodleXml,
    Qti12,
    Qti21,
    Qti22,
    Qti30,
}

impl std::fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportFormat::MoodleXml => write!(f, "Moodle XML"),
            ExportFormat::Qti12 => write!(f, "QTI 1.2"),
            ExportFormat::Qti21 => write!(f, "QTI 2.1"),
            ExportFormat::Qti22 => write!(f, "QTI 2.2"),
            ExportFormat::Qti30 => write!(f, "QTI 3.0"),
        }
    }
}

impl ExportFormat {
    /// このフォーマットの推奨ファイル拡張子を返す
    pub fn file_extension(&self) -> &'static str {
        match self {
            ExportFormat::MoodleXml => "xml",
            ExportFormat::Qti12 => "xml",
            ExportFormat::Qti21 => "xml",
            ExportFormat::Qti22 => "xml",
            ExportFormat::Qti30 => "xml",
        }
    }
}

/// エクスポートハンドラーの共通トレイト
/// 各フォーマット（Moodle XML、QTI 1.2 等）はこのトレイトを実装する。
pub trait ExportHandler {
    /// 設問リストを指定のパスにエクスポートする。
    ///
    /// # Arguments
    /// * `questions` - エクスポートする設問リスト
    /// * `subject` - 科目名
    /// * `output_path` - 出力先のパス
    ///
    /// # Returns
    /// 成功時は Ok(())、失敗時は Err(エラーメッセージ)
    fn export(
        &self,
        questions: &[QuestionWithChoices],
        subject: &str,
        output_path: &PathBuf,
    ) -> anyhow::Result<()>;

    /// このハンドラーのフォーマット名を返す
    fn format_name(&self) -> &'static str;
}

/// 指定フォーマットに対応するエクスポーターを生成する
pub fn create_exporter(format: ExportFormat) -> Box<dyn ExportHandler> {
    match format {
        ExportFormat::MoodleXml => Box::new(MoodleExporter),
        ExportFormat::Qti12 => Box::new(Qti12Exporter),
        ExportFormat::Qti21 => Box::new(Qti21Exporter),
        ExportFormat::Qti22 => Box::new(Qti22Exporter),
        ExportFormat::Qti30 => Box::new(Qti30Exporter),
    }
}
