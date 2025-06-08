//! シラバス検索フォーム関連のモジュール
//!
//! UNIVERSAL PASSPORT EXのシラバス検索フォームに関連する機能を提供します。
//! 検索条件の設定や構築を行います。

/// ビルダーパターンで検索フォームを構築
pub mod builder;

/// 検索フォームのパーサー（将来の拡張用）
pub mod parser;

/// 検索フォームのデータモデル
pub mod model;

// 公開API
pub use builder::SyllabusSearchFormBuilder;
pub use model::SyllabusSearchForm;
pub use parser::SyllabusSearchFormParser;
