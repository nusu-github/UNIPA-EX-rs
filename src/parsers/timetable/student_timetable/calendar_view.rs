use scraper::Html;

use super::StudentTimetable;
use crate::common::traits::PageParser;
use crate::utils::error::ParseError;

pub struct CalendarViewParser;

impl CalendarViewParser {
    pub fn new() -> Result<Self, ParseError> {
        Ok(Self)
    }
}

impl PageParser<StudentTimetable> for CalendarViewParser {
    const PAGE_TYPE: &'static str = "学生時間割表 表示形式：カレンダー";

    fn parse_document(&self, document: &Html) -> Result<StudentTimetable, ParseError> {
        let _ = document; // suppress unused variable warnings
        Err(ParseError::NotImplemented {
            operation: "StudentTimetable calendar view parsing".to_string(),
        })
    }
}
