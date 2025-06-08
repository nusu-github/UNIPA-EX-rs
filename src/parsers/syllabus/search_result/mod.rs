//! シラバス検索結果関連のモジュール
//!
//! UNIVERSAL PASSPORT EXのシラバス検索結果ページに関連する機能を提供します。
//! 検索結果の解析、ページネーション、フォーム情報の取得を行います。

/// ビルダーパターンで検索結果パーサーを構築
pub mod builder;

/// 検索結果のパーサー実装
pub mod parser;

/// 検索結果のデータモデル
pub mod model;

// 公開API
pub use builder::SyllabusSearchResultParserBuilder;
pub use model::*;
pub use parser::SyllabusSearchResultParser;
