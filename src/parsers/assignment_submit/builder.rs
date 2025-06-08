//! 課題提出一覧パーサービルダー
//!
//! 課題提出一覧パーサーの設定を管理し、パーサーインスタンスを構築します。

use scraper::Html;

use super::model::AssignmentList;
use super::parser::AssignmentListParserImpl;
use crate::common::traits::PageParser;
use crate::utils::error::ParseError;

/// 課題提出一覧パーサービルダー
pub struct AssignmentListParserBuilder {
    /// デバッグモード
    debug_mode: bool,
    /// 厳密なパースモード
    strict_mode: bool,
}

impl AssignmentListParserBuilder {
    /// 新しいパーサービルダーインスタンスを作成
    pub fn new() -> Self {
        Self {
            debug_mode: false,
            strict_mode: false,
        }
    }

    /// デバッグモードを設定
    pub fn with_debug_mode(mut self, debug_mode: bool) -> Self {
        self.debug_mode = debug_mode;
        self
    }

    /// 厳密なパースモードを設定
    pub fn with_strict_mode(mut self, strict_mode: bool) -> Self {
        self.strict_mode = strict_mode;
        self
    }

    /// 課題提出一覧パーサーを構築
    pub fn build(&self) -> Result<AssignmentListParserImpl, ParseError> {
        Ok(AssignmentListParserImpl::new_with_config(
            self.debug_mode,
            self.strict_mode,
        ))
    }
}

impl PageParser<AssignmentList> for AssignmentListParserBuilder {
    const PAGE_TYPE: &'static str = "課題提出一覧";

    fn parse_document(&self, document: &Html) -> Result<AssignmentList, ParseError> {
        let parser = self.build()?;
        parser.parse_document(document)
    }
}
