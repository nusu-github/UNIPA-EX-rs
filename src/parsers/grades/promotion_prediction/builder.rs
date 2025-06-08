//! 進級見込判定パーサービルダー

use crate::common::traits::PageParser;
use crate::utils::error::ParseError;
use scraper::Html;

use super::model::PromotionPredictionData;
use super::parser::PromotionPredictionParserImpl;

/// 進級見込判定パーサービルダー
pub struct PromotionPredictionParserBuilder {
    // 現在は設定項目なし、将来的に拡張可能
}

impl PromotionPredictionParserBuilder {
    /// 新しいパーサービルダーインスタンスを作成
    pub fn new() -> Self {
        Self {}
    }

    /// 進級見込判定パーサーを構築
    pub fn build(&self) -> Result<PromotionPredictionParserImpl, ParseError> {
        Ok(PromotionPredictionParserImpl::new())
    }
}

impl PageParser<PromotionPredictionData> for PromotionPredictionParserBuilder {
    const PAGE_TYPE: &'static str = "進級見込判定";

    fn parse_document(&self, document: &Html) -> Result<PromotionPredictionData, ParseError> {
        let parser = self.build()?;
        parser.parse_document(document)
    }
}

impl Default for PromotionPredictionParserBuilder {
    fn default() -> Self {
        Self::new()
    }
}
