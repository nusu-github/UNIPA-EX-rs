//! 学科開講一覧のカレンダー表示パーサー
//!
//! UNIVERSAL PASSPORT EXの学科開講一覧ページのカレンダー表示形式を解析します。
//! 週間カレンダー形式で表示される授業情報を構造化して抽出します。

use scraper::Html;

use super::super::{BaseClassInfo, DisplayFormat, ScheduleData};
use crate::common::traits::PageParser;
use crate::utils::error::ParseError;

/// 学科開講一覧のカレンダー表示パーサー
pub struct CalendarViewParser;

impl CalendarViewParser {
    /// 新しいパーサーインスタンスを作成
    pub fn new() -> Result<Self, ParseError> {
        Ok(Self)
    }
}

/// 学科開講一覧の授業エントリ（カレンダー表示用）
pub struct CourseEntry {
    /// 基本授業情報
    pub base: BaseClassInfo,
}

/// 学科開講一覧の全体構造（カレンダー表示）
pub struct CourseTimetableCalendar {
    /// 学生情報ラベル（大学・学部など判別用）
    pub student_info_label: String,
    /// 開講年度
    pub opening_year: u32,
    /// 学期
    pub semester: String,
    /// 表示形式（常にCalendar）
    pub display_format: DisplayFormat,
    /// カレンダー形式のスケジュールデータ
    pub schedule: ScheduleData<CourseEntry>,
}

impl PageParser<CourseTimetableCalendar> for CalendarViewParser {
    const PAGE_TYPE: &'static str = "学科開講一覧 表示形式：カレンダー";

    fn parse_document(&self, document: &Html) -> Result<CourseTimetableCalendar, ParseError> {
        todo!()
    }
}
