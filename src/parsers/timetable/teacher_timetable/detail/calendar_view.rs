//! 教員時間割のカレンダー表示パーサー
//!
//! UNIVERSAL PASSPORT EXの教員時間割詳細ページのカレンダー表示形式を解析します。
//! 週間カレンダー形式で表示される担当授業情報を構造化して抽出します。

use scraper::Html;

use super::super::super::{BaseClassInfo, DisplayFormat, ScheduleData};
use crate::common::traits::PageParser;
use crate::utils::error::ParseError;

/// 教員時間割のカレンダー表示パーサー
pub struct CalendarViewParser;

impl CalendarViewParser {
    /// 新しいパーサーインスタンスを作成
    pub fn new() -> Result<Self, ParseError> {
        Ok(Self)
    }
}

/// 教員時間割の授業エントリ（カレンダー表示用）
pub struct TeacherClassEntry {
    /// 基本授業情報
    pub base: BaseClassInfo,
    /// 履修者数（ある場合）
    pub enrollment_count: Option<u32>,
}

/// 教員時間割の全体構造（カレンダー表示）
pub struct TeacherTimetableCalendar {
    /// 教員名
    pub teacher_name: String,
    /// 開講年度
    pub opening_year: u32,
    /// 学期
    pub semester: String,
    /// 表示形式（常にCalendar）
    pub display_format: DisplayFormat,
    /// カレンダー形式のスケジュールデータ
    pub schedule: ScheduleData<TeacherClassEntry>,
}

impl PageParser<TeacherTimetableCalendar> for CalendarViewParser {
    const PAGE_TYPE: &'static str = "教員時間割表 表示形式：カレンダー";

    fn parse_document(&self, document: &Html) -> Result<TeacherTimetableCalendar, ParseError> {
        todo!()
    }
}
