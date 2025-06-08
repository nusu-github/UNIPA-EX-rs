//! 成績照会パーサービルダー
//!
//! 成績照会パーサーの設定を管理し、パーサーインスタンスを生成します。

use crate::common::traits::PageParser;
use crate::utils::error::ParseError;
use scraper::Html;

use super::model::{DisplayPattern, DisplaySettings, GradeInquiry};
use super::parser::GradeInquiryParserImpl;

/// 成績照会パーサービルダー
pub struct GradeInquiryParserBuilder {
    display_pattern: DisplayPattern,
    display_settings: DisplaySettings,
}

impl GradeInquiryParserBuilder {
    /// 新しいパーサービルダーインスタンスを作成
    pub fn new() -> Self {
        Self {
            display_pattern: DisplayPattern::default(),
            display_settings: DisplaySettings::default(),
        }
    }

    /// 表示パターンを設定
    pub fn with_display_pattern(mut self, display_pattern: DisplayPattern) -> Self {
        self.display_pattern = display_pattern;
        self
    }

    /// 表示設定を設定
    pub fn with_display_settings(mut self, display_settings: DisplaySettings) -> Self {
        self.display_settings = display_settings;
        self
    }

    /// 成績照会パーサーを構築
    pub fn build(&self) -> Result<GradeInquiryParserImpl, ParseError> {
        Ok(GradeInquiryParserImpl::new(
            self.display_pattern.clone(),
            self.display_settings.clone(),
        ))
    }
}

impl PageParser<GradeInquiry> for GradeInquiryParserBuilder {
    const PAGE_TYPE: &'static str = "成績照会";

    fn parse_document(&self, document: &Html) -> Result<GradeInquiry, ParseError> {
        let parser = self.build()?;
        parser.parse_document(document)
    }
}

impl Default for GradeInquiryParserBuilder {
    fn default() -> Self {
        Self::new()
    }
}
