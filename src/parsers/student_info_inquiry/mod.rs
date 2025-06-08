//! 学籍情報照会関連のページパーサー
//!
//! UNIVERSAL PASSPORT EXの学籍情報照会機能に関連するページを解析します。
//! 学生の基本情報、所属情報、指導教員情報、学籍変更履歴などの機能をサポートします。

/// 学籍情報照会のデータモデル
pub mod model;

/// 学籍情報照会パーサー実装
pub mod parser;

/// 学籍情報照会パーサービルダー
pub mod builder;

// 後方互換性のため、既存のstudent_info_inquiryモジュールをエクスポート
pub mod student_info_inquiry {
    pub use super::builder::StudentInfoInquiryParserBuilder as StudentInfoInquiryParser;
    pub use super::model::*;
}

use crate::common::traits::PageParser;
use scraper::Html;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct StudentInfoInquiryParser {
    parser: builder::StudentInfoInquiryParserBuilder,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl StudentInfoInquiryParser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            parser: builder::StudentInfoInquiryParserBuilder::new(),
        }
    }

    pub fn parse(&self, html_content: &str) -> Result<model::StudentInfo, JsError> {
        Ok(self
            .parser
            .parse_document(&Html::parse_document(html_content))?)
    }
}
