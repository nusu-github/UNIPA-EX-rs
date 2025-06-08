//! 課題提出関連のページパーサー
//!
//! UNIVERSAL PASSPORT EXの課題提出機能に関連するページを解析します。
//! 課題一覧、提出フォーム、提出状況などの機能をサポートします。

/// 課題提出一覧のデータモデル
pub mod model;

/// 課題提出一覧パーサー実装
pub mod parser;

/// 課題提出一覧パーサービルダー
pub mod builder;

// 後方互換性のため、既存のassignment_listモジュールをエクスポート
pub mod assignment_list {
    pub use super::builder::AssignmentListParserBuilder as AssignmentListParser;
    pub use super::model::*;
}

use crate::common::traits::PageParser;
use scraper::Html;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct AssignmentSubmitParser {
    assignment_list: builder::AssignmentListParserBuilder,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl AssignmentSubmitParser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            assignment_list: builder::AssignmentListParserBuilder::new(),
        }
    }

    pub fn assignment_list(&self, html_content: &str) -> Result<model::AssignmentList, JsError> {
        Ok(self
            .assignment_list
            .parse_document(&Html::parse_document(html_content))?)
    }
}
