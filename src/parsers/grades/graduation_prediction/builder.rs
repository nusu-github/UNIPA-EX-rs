//! 卒業見込判定パーサービルダー

use crate::common::traits::PageParser;
use crate::utils::error::ParseError;
use scraper::Html;

use super::model::SotsugyoMikonHanteiKekka;
use super::parser::GraduationPredictionParserImpl;

/// 卒業見込判定パーサービルダー
pub struct GraduationPredictionParserBuilder {
    // 現在は設定項目なし、将来的に拡張可能
}

impl GraduationPredictionParserBuilder {
    /// 新しいパーサービルダーインスタンスを作成
    pub fn new() -> Self {
        Self {}
    }

    /// 卒業見込判定パーサーを構築
    pub fn build(&self) -> Result<GraduationPredictionParserImpl, ParseError> {
        Ok(GraduationPredictionParserImpl::new())
    }
}

impl PageParser<SotsugyoMikonHanteiKekka> for GraduationPredictionParserBuilder {
    const PAGE_TYPE: &'static str = "卒業見込判定";

    fn parse_document(&self, document: &Html) -> Result<SotsugyoMikonHanteiKekka, ParseError> {
        let parser = self.build()?;
        parser.parse_document(document)
    }
}

impl Default for GraduationPredictionParserBuilder {
    fn default() -> Self {
        Self::new()
    }
}
