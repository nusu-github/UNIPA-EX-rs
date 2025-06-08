//! 教室予約状況パーサービルダー
//!
//! 教室予約状況パーサーの設定を管理し、パーサーインスタンスを構築します。

use scraper::Html;

use super::model::ClassroomReservationStatus;
use super::parser::ClassroomReservationStatusParserImpl;
use crate::common::traits::PageParser;
use crate::utils::error::ParseError;

/// 教室予約状況パーサービルダー
pub struct ClassroomReservationStatusParserBuilder {
    /// デバッグモード
    debug_mode: bool,
    /// 厳密なパースモード
    strict_mode: bool,
}

impl ClassroomReservationStatusParserBuilder {
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

    /// 教室予約状況パーサーを構築
    pub fn build(&self) -> Result<ClassroomReservationStatusParserImpl, ParseError> {
        Ok(ClassroomReservationStatusParserImpl::new_with_config(
            self.debug_mode,
            self.strict_mode,
        ))
    }
}

impl PageParser<ClassroomReservationStatus> for ClassroomReservationStatusParserBuilder {
    const PAGE_TYPE: &'static str = "教室予約状況";

    fn parse_document(&self, document: &Html) -> Result<ClassroomReservationStatus, ParseError> {
        let parser = self.build()?;
        parser.parse_document(document)
    }
}
