//! シラバス詳細表示関連のモジュール
//!
//! UNIVERSAL PASSPORT EXのシラバス詳細表示ページに関連する機能を提供します。
//! 授業の詳細情報、計画、評価方法などの解析を行います。

/// ビルダーパターンで詳細表示パーサーを構築
pub mod builder;

/// 詳細表示のパーサー実装
pub mod parser;

/// 詳細表示のデータモデル
pub mod model;

// 公開API
pub use builder::SyllabusViewParserBuilder;
pub use model::*;
pub use parser::SyllabusViewParser;
