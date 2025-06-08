//! 卒業見込判定モジュール
//!
//! UNIVERSAL PASSPORT EXの卒業見込判定ページのパーサー機能を提供します。
//! 三層アーキテクチャ（Builder → Parser → Model）を採用しています。

pub mod builder;
pub mod model;
pub mod parser;

pub use builder::GraduationPredictionParserBuilder;
pub use model::{SotsugyoMikonHanteiKekka, YokenFusokuItem};
pub use parser::GraduationPredictionParserImpl;
