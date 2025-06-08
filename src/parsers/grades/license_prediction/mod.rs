//! 免許取得見込判定モジュール
//!
//! UNIVERSAL PASSPORT EXの免許取得見込判定ページのパーサー機能を提供します。
//! 三層アーキテクチャ（Builder → Parser → Model）を採用しています。
//!
//! 注意: このモジュールは現在プレースホルダ実装です。

pub mod builder;
pub mod model;
pub mod parser;

pub use builder::LicensePredictionParserBuilder;
pub use model::LicensePredictionData;
pub use parser::LicensePredictionParserImpl;
