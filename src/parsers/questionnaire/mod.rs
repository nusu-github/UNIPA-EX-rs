//! アンケート関連のページパーサー
//!
//! UNIVERSAL PASSPORT EXのアンケート機能に関連するページを解析します。
//! 学生向けのアンケート一覧表示と回答機能をサポートします。

/// アンケート一覧（回答可能なアンケートの表示）
pub mod questionnaire_list;

use crate::common::traits::PageParser;
use scraper::Html;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct QuestionnaireParser {
    questionnaire_list: questionnaire_list::QuestionnaireListParserBuilder,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl QuestionnaireParser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            questionnaire_list: questionnaire_list::QuestionnaireListParserBuilder::new(),
        }
    }

    pub fn questionnaire_list(
        &self,
        html_content: &str,
    ) -> Result<questionnaire_list::QuestionnaireList, JsError> {
        Ok(self
            .questionnaire_list
            .parse_document(&Html::parse_document(html_content))?)
    }
}
