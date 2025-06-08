//! 成績・単位関連のページパーサー
//!
//! UNIVERSAL PASSPORT EXの成績照会、卒業見込判定、進級見込判定、免許取得見込判定など、
//! 学習成果と単位に関連する機能のパーサーを提供します。

/// 成績照会（個別科目の成績とGPA）
pub mod grade_inquiry;

/// 卒業見込判定（卒業要件の充足状況）
pub mod graduation_prediction;

/// 免許取得見込判定（教員免許等の取得見込）
pub mod license_prediction;

/// 進級見込判定（進級要件の充足状況）
pub mod promotion_prediction;

use crate::common::traits::PageParser;
use scraper::Html;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct GradesParser {
    grade_inquiry: grade_inquiry::GradeInquiryParserBuilder,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl GradesParser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            grade_inquiry: grade_inquiry::GradeInquiryParserBuilder::new(),
        }
    }

    pub fn grade_inquiry(
        &self,
        html_content: &str,
    ) -> Result<grade_inquiry::GradeInquiry, JsError> {
        Ok(self
            .grade_inquiry
            .parse_document(&Html::parse_document(html_content))?)
    }
}
