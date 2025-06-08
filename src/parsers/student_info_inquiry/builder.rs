//! 学籍情報照会パーサービルダー
//!
//! 学籍情報照会パーサーの設定を管理し、パーサーインスタンスを構築します。

use scraper::Html;

use super::model::StudentInfo;
use super::parser::StudentInfoInquiryParserImpl;
use crate::common::traits::PageParser;
use crate::utils::error::ParseError;

/// 学籍情報照会パーサービルダー
pub struct StudentInfoInquiryParserBuilder;

impl StudentInfoInquiryParserBuilder {
    /// 新しいパーサービルダーインスタンスを作成
    pub fn new() -> Self {
        Self
    }

    /// 学籍情報照会パーサーを構築
    pub fn build(&self) -> Result<StudentInfoInquiryParserImpl, ParseError> {
        Ok(StudentInfoInquiryParserImpl::new_with_config())
    }
}

impl PageParser<StudentInfo> for StudentInfoInquiryParserBuilder {
    const PAGE_TYPE: &'static str = "学籍情報照会";

    fn parse_document(&self, document: &Html) -> Result<StudentInfo, ParseError> {
        let parser = self.build()?;
        parser.parse_document(document)
    }
}
