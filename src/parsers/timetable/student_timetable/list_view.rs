//! 学生時間割の一覧表示パーサー
//!
//! UNIVERSAL PASSPORT EXの学生時間割ページの一覧表示形式を解析します。
//! テーブル形式で表示される履修科目情報、単位取得状況、
//! エラー情報を構造化して抽出します。

use scraper::Html;

use super::StudentTimetable;
use crate::common::traits::PageParser;
use crate::utils::error::ParseError;

/// 学生時間割の一覧表示パーサー
pub struct ListViewParser;

impl ListViewParser {
    /// 新しいパーサーインスタンスを作成
    pub fn new() -> Result<Self, ParseError> {
        Ok(Self)
    }
}

impl PageParser<StudentTimetable> for ListViewParser {
    const PAGE_TYPE: &'static str = "学生時間割表 表示形式：一覧";

    fn parse_document(&self, document: &Html) -> Result<StudentTimetable, ParseError> {
        todo!()
    }
}
