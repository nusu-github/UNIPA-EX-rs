//! 卒業見込判定パーサー実装

use scraper::{Html, Selector};

use crate::utils::error::ParseError;

use super::model::{SotsugyoMikonHanteiKekka, YokenFusokuItem};

/// 卒業見込判定パーサー実装
pub struct GraduationPredictionParserImpl {}

impl GraduationPredictionParserImpl {
    /// 新しいパーサーインスタンスを作成
    pub fn new() -> Self {
        Self {}
    }

    /// HTMLドキュメントから卒業見込判定を解析する
    pub fn parse_document(&self, document: &Html) -> Result<SotsugyoMikonHanteiKekka, ParseError> {
        let hantei_message = self.parse_judgement_message(document)?;
        let fusoku_items = self.parse_fusoku_items(document)?;

        Ok(SotsugyoMikonHanteiKekka {
            hantei_message,
            fusoku_items,
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

    /// 要件不足項目リストを解析する
    fn parse_fusoku_items(&self, document: &Html) -> Result<Vec<YokenFusokuItem>, ParseError> {
        let mut fusoku_items = Vec::new();

        // 不足項目テーブルを探す
        let table_selector = Selector::parse("table.fusoku, .requirements-table").map_err(|e| {
            ParseError::selector_creation_failed(
                "table.fusoku, .requirements-table",
                &e.to_string(),
            )
        })?;

        if let Some(table) = document.select(&table_selector).next() {
            let row_selector = Selector::parse("tbody tr")
                .map_err(|e| ParseError::selector_creation_failed("tbody tr", &e.to_string()))?;

            let cell_selector = Selector::parse("td")
                .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

            for row in table.select(&row_selector) {
                let cells: Vec<_> = row.select(&cell_selector).collect();

                if cells.len() >= 4 {
                    let joken_code = cells[0].inner_html().trim().to_string();
                    let yoso_number = cells[1].inner_html().trim().parse::<i32>().unwrap_or(0);
                    let fusoku_message = cells[2].inner_html().trim().to_string();
                    let fusoku_ryo = cells[3].inner_html().trim().to_string();

                    fusoku_items.push(YokenFusokuItem {
                        joken_code,
                        yoso_number,
                        fusoku_message,
                        fusoku_ryo,
                    });
                }
            }
        }

        Ok(fusoku_items)
    }
}
