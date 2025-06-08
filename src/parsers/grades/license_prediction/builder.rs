//! 免許取得見込判定パーサービルダー（プレースホルダ実装）

use crate::common::traits::PageParser;
use crate::utils::error::ParseError;
use scraper::Html;

use super::model::LicensePredictionData;
use super::parser::LicensePredictionParserImpl;

/// 免許取得見込判定パーサービルダー
///
/// 注意: この実装は現在プレースホルダです。
pub struct LicensePredictionParserBuilder {
    // 現在は設定項目なし、将来的に拡張可能
}

impl LicensePredictionParserBuilder {
    /// 新しいパーサービルダーインスタンスを作成
    pub fn new() -> Self {
        Self {}
    }

    /// 免許取得見込判定パーサーを構築
    pub fn build(&self) -> Result<LicensePredictionParserImpl, ParseError> {
        Ok(LicensePredictionParserImpl::new())
    }
}

impl PageParser<LicensePredictionData> for LicensePredictionParserBuilder {
    const PAGE_TYPE: &'static str = "免許取得見込判定";

    fn parse_document(&self, document: &Html) -> Result<LicensePredictionData, ParseError> {
        let parser = self.build()?;
        parser.parse_document(document)
    }
}

impl Default for LicensePredictionParserBuilder {
    fn default() -> Self {
        Self::new()
    }
}
