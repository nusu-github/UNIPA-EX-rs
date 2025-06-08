//! シラバス検索フォームのパーサー
//!
//! HTMLからシラバス検索フォームの情報を抽出します。
//! 現在はパースロジックは実装されていませんが、将来の拡張用として構造を用意しています。

use super::model::SyllabusSearchForm;
use crate::common::traits::PageParser;
use crate::utils::error::ParseError;
use scraper::Html;

/// シラバス検索フォームパーサー
///
/// HTMLからシラバス検索フォームの情報を抽出する機能を提供します。
/// 現在は基本的な構造のみ実装されており、将来の拡張を想定しています。
pub struct SyllabusSearchFormParser;

impl SyllabusSearchFormParser {
    /// 新しいパーサーインスタンスを作成
    pub fn new() -> Result<Self, ParseError> {
        Ok(Self)
    }
}

impl PageParser<SyllabusSearchForm> for SyllabusSearchFormParser {
    const PAGE_TYPE: &'static str = "syllabus_search";

    /// HTMLドキュメントから検索フォーム情報を解析
    ///
    /// 現在は空の検索フォームを返します。
    /// 将来的には、HTMLから検索フォームの初期値やオプション等を抽出する機能を実装予定です。
    fn parse_document(&self, _document: &Html) -> Result<SyllabusSearchForm, ParseError> {
        // 現在はパースロジックなし - 将来の拡張用
        Ok(SyllabusSearchForm::default())
    }
}
