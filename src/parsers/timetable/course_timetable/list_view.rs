//! 学科開講一覧の一覧表示パーサー
//!
//! UNIVERSAL PASSPORT EXの学科開講一覧ページの一覧表示形式を解析します。
//! テーブル形式で表示される授業情報を構造化して抽出します。

use scraper::{Html, Selector};

use super::super::{BaseClassInfo, DisplayFormat, ScheduleData};
use crate::common::traits::PageParser;
use crate::utils::error::ParseError;

/// 学科開講一覧の一覧表示パーサー
pub struct ListViewParser;

impl ListViewParser {
    /// 新しいパーサーインスタンスを作成
    pub fn new() -> Result<Self, ParseError> {
        Ok(Self)
    }

    /// コース情報を解析する
    fn parse_course_entries(&self, document: &Html) -> Result<Vec<CourseEntry>, ParseError> {
        let table_selector = Selector::parse("#form1\\:table2")
            .map_err(|e| ParseError::selector_creation_failed("#form1:table2", &e.to_string()))?;

        let row_selector = Selector::parse("tbody tr")
            .map_err(|e| ParseError::selector_creation_failed("tbody tr", &e.to_string()))?;

        let cell_selector = Selector::parse("td")
            .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

        let table = document
            .select(&table_selector)
            .next()
            .ok_or_else(|| ParseError::element_not_found("#form1:table2", "メインテーブル"))?;

        let mut entries = Vec::new();

        for row in table.select(&row_selector) {
            let cells: Vec<_> = row.select(&cell_selector).collect();

            if cells.len() >= 5 {
                let opening_type = cells[0].inner_html().trim().to_string();
                let class_code = cells[1].inner_html().trim().to_string();
                let subject_name = self.extract_subject_name(&cells[2])?;
                let teacher_name = cells[3].inner_html().trim().to_string();
                let classroom_html = cells[4].inner_html();
                let classroom_text = classroom_html.trim();
                let classroom = {
                    if classroom_text.is_empty() {
                        None
                    } else {
                        Some(classroom_text.to_string())
                    }
                };

                entries.push(CourseEntry {
                    base: BaseClassInfo {
                        day_and_period: opening_type,
                        class_code,
                        subject_name,
                        teacher_name,
                        classroom,
                    },
                    credits: 0, // This might need to be extracted from additional data
                    enrollment_count: None, // This might need to be extracted from additional data
                });
            }
        }

        Ok(entries)
    }

    /// 科目名を抽出する（リンクが含まれる場合も考慮）
    fn extract_subject_name(&self, cell: &scraper::ElementRef) -> Result<String, ParseError> {
        let link_selector = Selector::parse("a")
            .map_err(|e| ParseError::selector_creation_failed("a", &e.to_string()))?;

        if let Some(link) = cell.select(&link_selector).next() {
            Ok(link.inner_html().trim().to_string())
        } else {
            Ok(cell.inner_html().trim().to_string())
        }
    }

    /// 学期情報を抽出する
    fn parse_semester_info(&self, document: &Html) -> Result<(u32, String), ParseError> {
        let _ = document; // suppress unused variable warnings
        Err(ParseError::NotImplemented {
            operation: "Course timetable semester info parsing".to_string(),
        })
    }

    /// 学生情報ラベルを抽出する
    fn parse_student_info_label(&self, document: &Html) -> Result<String, ParseError> {
        let _ = document; // suppress unused variable warnings
        Err(ParseError::NotImplemented {
            operation: "Course timetable student info label parsing".to_string(),
        })
    }
}

/// 学科開講一覧の授業エントリ（一覧表示用）
pub struct CourseEntry {
    /// 基本授業情報
    pub base: BaseClassInfo,
    /// 単位数
    pub credits: u32,
    /// 履修者数（ある場合）
    pub enrollment_count: Option<u32>,
}

/// 学科開講一覧の全体構造（一覧表示）
pub struct CourseTimetableList {
    /// 学生情報ラベル（大学・学部など判別用）
    pub student_info_label: String,
    /// 開講年度
    pub opening_year: u32,
    /// 学期
    pub semester: String,
    /// 表示形式（常にList）
    pub display_format: DisplayFormat,
    /// 一覧形式のスケジュールデータ
    pub schedule: ScheduleData<CourseEntry>,
    /// 集中講義・実習
    pub irregular_classes: Vec<CourseEntry>,
}

impl PageParser<CourseTimetableList> for ListViewParser {
    const PAGE_TYPE: &'static str = "学科開講一覧 表示形式：一覧";

    fn parse_document(&self, document: &Html) -> Result<CourseTimetableList, ParseError> {
        let (opening_year, semester) = self.parse_semester_info(document)?;
        let student_info_label = self.parse_student_info_label(document)?;
        let course_entries = self.parse_course_entries(document)?;

        // Separate regular classes from irregular (concentrated) classes
        let (regular_classes, irregular_classes): (Vec<_>, Vec<_>) = course_entries
            .into_iter()
            .partition(|entry| !entry.base.day_and_period.contains("集中"));

        Ok(CourseTimetableList {
            student_info_label,
            opening_year,
            semester,
            display_format: DisplayFormat::List,
            schedule: ScheduleData::List(regular_classes),
            irregular_classes,
        })
    }
}
