//! 進級見込判定パーサー実装

use scraper::{Html, Selector};

use crate::utils::error::ParseError;

use super::model::PromotionPredictionData;

/// 進級見込判定パーサー実装
pub struct PromotionPredictionParserImpl {}

impl PromotionPredictionParserImpl {
    /// 新しいパーサーインスタンスを作成
    pub fn new() -> Self {
        Self {}
    }

    /// HTMLドキュメントから進級見込判定を解析する
    pub fn parse_document(&self, document: &Html) -> Result<PromotionPredictionData, ParseError> {
        let judgement_message = self.parse_judgement_message(document)?;
        let last_search_student_id = self.parse_hidden_field(document, "lastSearchStudentId")?;
        let academic_year = self.parse_hidden_field(document, "academicYear")?;
        let semester = self.parse_hidden_field(document, "semester")?;

        Ok(PromotionPredictionData {
            judgement_message,
            last_search_student_id,
            academic_year,
            semester,
        })
    }

    /// 判定メッセージを解析する
    fn parse_judgement_message(&self, document: &Html) -> Result<String, ParseError> {
        // 判定結果メッセージエリアを探す
        let message_selector = Selector::parse(".result-message, .judgement-message, .message")
            .map_err(|e| {
                ParseError::selector_creation_failed(
                    ".result-message, .judgement-message, .message",
                    &e.to_string(),
                )
            })?;

        if let Some(message_element) = document.select(&message_selector).next() {
            return Ok(message_element.inner_html().trim().to_string());
        }

        // 見つからない場合はプレースホルダ
        Ok("判定結果を取得できませんでした。".to_string())
    }

    /// 隠しフィールドの値を解析する
    fn parse_hidden_field(&self, document: &Html, field_name: &str) -> Result<String, ParseError> {
        let input_selector =
            Selector::parse(&format!("input[name='{}']", field_name)).map_err(|e| {
                ParseError::selector_creation_failed(
                    &format!("input[name='{}']", field_name),
                    &e.to_string(),
                )
            })?;

        if let Some(input_element) = document.select(&input_selector).next() {
            if let Some(value) = input_element.value().attr("value") {
                return Ok(value.to_string());
            }
        }

        // 見つからない場合は空文字列
        Ok(String::new())
    }
}
