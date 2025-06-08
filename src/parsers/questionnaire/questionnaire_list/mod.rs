//! アンケート一覧パーサーモジュール
//!
//! 三層アーキテクチャに基づいて実装されています：
//! - Builder: パーサーの設定とインスタンス作成
//! - Parser: HTMLパースロジック
//! - Model: データ構造の定義

pub mod builder;
pub mod model;
pub mod parser;

// 公開API
pub use builder::QuestionnaireListParserBuilder;
pub use model::*;
pub use parser::QuestionnaireListParserImpl;

// 後方互換性のためのエイリアス
pub type QuestionnaireListParser = QuestionnaireListParserBuilder;
