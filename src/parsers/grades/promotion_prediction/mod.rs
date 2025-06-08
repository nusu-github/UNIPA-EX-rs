//! 進級見込判定モジュール
//!
//! UNIVERSAL PASSPORT EXの進級見込判定ページのパーサー機能を提供します。
//! 三層アーキテクチャ（Builder → Parser → Model）を採用しています。

pub mod builder;
pub mod model;
pub mod parser;

pub use builder::PromotionPredictionParserBuilder;
pub use model::PromotionPredictionData;
pub use parser::PromotionPredictionParserImpl;
