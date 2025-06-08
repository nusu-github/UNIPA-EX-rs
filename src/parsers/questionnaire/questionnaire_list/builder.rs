//! アンケート一覧パーサービルダー
//!
//! パーサーの設定管理とインスタンス作成を担当します。

use scraper::Html;

use crate::common::traits::PageParser;
use crate::utils::error::ParseError;

use super::model::QuestionnaireList;
use super::parser::QuestionnaireListParserImpl;

/// アンケート一覧パーサービルダー
///
/// パーサーの設定値を管理し、適切に設定されたパーサーインスタンスを作成します。
pub struct QuestionnaireListParserBuilder;

impl QuestionnaireListParserBuilder {
    /// 新しいパーサービルダーインスタンスを作成
    ///
    /// # 戻り値
    ///
    /// 新しいパーサービルダーインスタンス
    pub fn new() -> Self {
        Self
    }

    /// アンケート一覧パーサーを構築
    ///
    /// # 戻り値
    ///
    /// 設定された値に基づくパーサーインスタンス
    ///
    /// # エラー
    ///
    /// パーサーの構築に失敗した場合は `ParseError` を返します。
    pub fn build(&self) -> Result<QuestionnaireListParserImpl, ParseError> {
        Ok(QuestionnaireListParserImpl::new())
    }
}

impl PageParser<QuestionnaireList> for QuestionnaireListParserBuilder {
    const PAGE_TYPE: &'static str = "アンケート一覧";

    fn parse_document(&self, document: &Html) -> Result<QuestionnaireList, ParseError> {
        let parser = self.build()?;
        parser.parse_document(document)
    }
}
