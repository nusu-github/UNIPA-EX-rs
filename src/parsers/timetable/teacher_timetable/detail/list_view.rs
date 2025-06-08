//! 教員時間割の一覧表示パーサー
//!
//! UNIVERSAL PASSPORT EXの教員時間割詳細ページの一覧表示形式を解析します。
//! テーブル形式で表示される担当授業情報を構造化して抽出します。

use scraper::Html;

use super::super::super::{BaseClassInfo, DisplayFormat, ScheduleData};
use crate::common::traits::PageParser;
use crate::utils::error::ParseError;

/// 教員時間割の一覧表示パーサー
pub struct ListViewParser;

impl ListViewParser {
    /// 新しいパーサーインスタンスを作成
    pub fn new() -> Result<Self, ParseError> {
        Ok(Self)
    }
}

/// 教員時間割の授業エントリ（一覧表示用）
pub struct TeacherClassEntry {
    /// 基本授業情報
    pub base: BaseClassInfo,
    /// 単位数
    pub credits: u32,
    /// 履修者数（ある場合）
    pub enrollment_count: Option<u32>,
    /// 対象学科・学年
    pub target_department_grade: String,
}

/// 教員時間割の全体構造（一覧表示）
pub struct TeacherTimetableList {
    /// 教員名
    pub teacher_name: String,
    /// 開講年度
    pub opening_year: u32,
    /// 学期
    pub semester: String,
    /// 表示形式（常にList）
    pub display_format: DisplayFormat,
    /// 一覧形式のスケジュールデータ
    pub schedule: ScheduleData<TeacherClassEntry>,
    /// 集中講義・実習
    pub irregular_classes: Vec<TeacherClassEntry>,
}

impl PageParser<TeacherTimetableList> for ListViewParser {
    const PAGE_TYPE: &'static str = "教員時間割表 表示形式：一覧";

    fn parse_document(&self, document: &Html) -> Result<TeacherTimetableList, ParseError> {
        todo!()
    }
}
