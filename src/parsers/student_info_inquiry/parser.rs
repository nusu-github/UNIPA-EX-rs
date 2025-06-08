//! 学籍情報照会パーサー実装
//!
//! HTMLドキュメントから学生の学籍情報を抽出するパースロジックを提供します。

use scraper::{Html, Selector};

use super::model::{AdvisorInfo, AffiliationInfo, BasicInfo, StatusChangeInfo, StudentInfo};
use crate::utils::error::ParseError;

/// 学籍情報照会パーサー実装
pub struct StudentInfoInquiryParserImpl;

impl StudentInfoInquiryParserImpl {
    /// 新しいパーサーインスタンスを作成
    pub fn new_with_config() -> Self {
        Self
    }

    /// HTMLドキュメントから学生情報を解析する
    pub fn parse_document(&self, document: &Html) -> Result<StudentInfo, ParseError> {
        let basic_info = self.parse_basic_info(document)?;
        let affiliation_info = self.parse_affiliation_info(document)?;
        let advisor_info = self.parse_advisor_info(document)?;
        let status_change_info = self.parse_status_change_info(document)?;

        Ok(StudentInfo {
            basic_info,
            affiliation_info,
            advisor_info,
            status_change_info,
        })
    }

    /// 基本情報セクションをパースする
    fn parse_basic_info(&self, document: &Html) -> Result<BasicInfo, ParseError> {
        let base_table_selector = Selector::parse("#baseTable")
            .map_err(|e| ParseError::selector_creation_failed("#baseTable", &e.to_string()))?;

        let row_selector = Selector::parse("tr")
            .map_err(|e| ParseError::selector_creation_failed("tr", &e.to_string()))?;

        let th_selector = Selector::parse("th")
            .map_err(|e| ParseError::selector_creation_failed("th", &e.to_string()))?;

        let td_selector = Selector::parse("td")
            .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

        let base_table = document
            .select(&base_table_selector)
            .next()
            .ok_or_else(|| ParseError::element_not_found("#baseTable", "基本情報テーブル"))?;

        let mut basic_info = BasicInfo::default();

        for row in base_table.select(&row_selector) {
            let th = row.select(&th_selector).next();
            let td = row.select(&td_selector).next();

            if let (Some(th), Some(td)) = (th, td) {
                let label = th.inner_html().trim().to_string();
                let value = td.inner_html().trim().to_string();

                match label.as_str() {
                    "学籍番号" => basic_info.student_id = value,
                    "学生氏名" => basic_info.student_name = value,
                    "カナ氏名" => basic_info.kana_name = value,
                    "性別" => basic_info.gender = value,
                    "生年月日" => basic_info.date_of_birth = self.format_date(&value)?,
                    "国籍" => {
                        basic_info.nationality = if value.is_empty() { None } else { Some(value) }
                    }
                    "PCメールアドレス" => {
                        basic_info.pc_email_address =
                            if value.is_empty() { None } else { Some(value) }
                    }
                    "入学種別" => basic_info.enrollment_type = value,
                    "就学種別" => basic_info.student_status_type = value,
                    "入学年度" => basic_info.enrollment_year = value.parse().unwrap_or(0),
                    "入学期NO" => basic_info.enrollment_term_no = value.parse().unwrap_or(0),
                    "カリキュラム対象年度" => {
                        basic_info.curriculum_target_year = value.parse().unwrap_or(0)
                    }
                    "カリキュラム対象学期" => {
                        basic_info.curriculum_target_term = value.parse().unwrap_or(0)
                    }
                    "入学日付" => basic_info.enrollment_date = self.format_date(&value)?,
                    "出学日付" => {
                        basic_info.withdrawal_date = if value.is_empty() {
                            None
                        } else {
                            Some(self.format_date(&value)?)
                        }
                    }
                    "卒業予定年月" => {
                        basic_info.expected_graduation_month_year =
                            self.format_year_month(&value)?
                    }
                    "修了予定日" => {
                        basic_info.completion_date = if value.is_empty() {
                            None
                        } else {
                            Some(self.format_date(&value)?)
                        }
                    }
                    _ => {} // Unknown field, ignore
                }
            }
        }

        Ok(basic_info)
    }

    /// 所属情報セクションをパースする
    fn parse_affiliation_info(&self, document: &Html) -> Result<AffiliationInfo, ParseError> {
        let selector = Selector::parse(".subTitleArea")
            .map_err(|e| ParseError::selector_creation_failed(".subTitleArea", &e.to_string()))?;

        let row_selector = Selector::parse("tr")
            .map_err(|e| ParseError::selector_creation_failed("tr", &e.to_string()))?;

        let th_selector = Selector::parse("th")
            .map_err(|e| ParseError::selector_creation_failed("th", &e.to_string()))?;

        let td_selector = Selector::parse("td")
            .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

        // Find the 所属情報 section - look for tables after the subtitle
        let mut affiliation_info = AffiliationInfo::default();

        for element in document.select(&selector) {
            if element.inner_html().trim() == "所属情報" {
                // Find the next table after this subtitle
                let mut current = element.next_sibling();
                while let Some(node) = current {
                    if let Some(table_element) = scraper::ElementRef::wrap(node) {
                        if table_element.value().name() == "table"
                            || table_element.select(&row_selector).next().is_some()
                        {
                            // Found a table, parse its rows
                            for row in table_element.select(&row_selector) {
                                let th = row.select(&th_selector).next();
                                let td = row.select(&td_selector).next();

                                if let (Some(th), Some(td)) = (th, td) {
                                    let label = th.inner_html().trim().to_string();
                                    let value = td.inner_html().trim().to_string();

                                    match label.as_str() {
                                        "所属学科組織" => {
                                            affiliation_info.affiliated_department_organization =
                                                value
                                        }
                                        "カリキュラム学科組織" => {
                                            affiliation_info.curriculum_department_organization =
                                                value
                                        }
                                        "学年" => {
                                            affiliation_info.grade_level =
                                                value.parse().unwrap_or(0)
                                        }
                                        "セメスタ" => {
                                            affiliation_info.semester = value.parse().unwrap_or(0)
                                        }
                                        "専攻コース" => {
                                            affiliation_info.major_course =
                                                if value.is_empty() { None } else { Some(value) }
                                        }
                                        "クラス種別＋クラス" => {
                                            affiliation_info.class_type_class = value
                                        }
                                        _ => {} // Unknown field, ignore
                                    }
                                }
                            }
                            break;
                        }
                    }
                    current = node.next_sibling();
                }
                break;
            }
        }

        Ok(affiliation_info)
    }

    /// 指導教員情報セクションをパースする
    fn parse_advisor_info(&self, document: &Html) -> Result<AdvisorInfo, ParseError> {
        let selector = Selector::parse(".subTitleArea")
            .map_err(|e| ParseError::selector_creation_failed(".subTitleArea", &e.to_string()))?;

        let row_selector = Selector::parse("tr")
            .map_err(|e| ParseError::selector_creation_failed("tr", &e.to_string()))?;

        let th_selector = Selector::parse("th")
            .map_err(|e| ParseError::selector_creation_failed("th", &e.to_string()))?;

        let td_selector = Selector::parse("td")
            .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

        // Find the 担当教員 section - look for tables after the subtitle
        let mut advisor_info = AdvisorInfo::default();

        for element in document.select(&selector) {
            if element.inner_html().trim() == "担当教員" {
                // Find the next table after this subtitle
                let mut current = element.next_sibling();
                while let Some(node) = current {
                    if let Some(table_element) = scraper::ElementRef::wrap(node) {
                        if table_element.value().name() == "table"
                            || table_element.select(&row_selector).next().is_some()
                        {
                            // Found a table, parse its rows
                            for row in table_element.select(&row_selector) {
                                let th = row.select(&th_selector).next();
                                let td = row.select(&td_selector).next();

                                if let (Some(th), Some(td)) = (th, td) {
                                    let label = th.inner_html().trim().to_string();
                                    let value =
                                        td.inner_html().trim().replace("<BR>", "").to_string();

                                    match label.as_str() {
                                        "担当教員名" => advisor_info.advisor_name = value,
                                        "担当開始日" => {
                                            advisor_info.advisor_start_date =
                                                self.format_date(&value)?
                                        }
                                        "担当終了日" => {
                                            advisor_info.advisor_end_date =
                                                self.format_date(&value)?
                                        }
                                        _ => {} // Unknown field, ignore
                                    }
                                }
                            }
                            break;
                        }
                    }
                    current = node.next_sibling();
                }
                break;
            }
        }

        Ok(advisor_info)
    }

    /// 学籍変更情報セクションをパースする
    fn parse_status_change_info(&self, document: &Html) -> Result<StatusChangeInfo, ParseError> {
        let selector = Selector::parse(".subTitleArea")
            .map_err(|e| ParseError::selector_creation_failed(".subTitleArea", &e.to_string()))?;

        let row_selector = Selector::parse("tr")
            .map_err(|e| ParseError::selector_creation_failed("tr", &e.to_string()))?;

        let th_selector = Selector::parse("th")
            .map_err(|e| ParseError::selector_creation_failed("th", &e.to_string()))?;

        let td_selector = Selector::parse("td")
            .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

        // Find the 異動情報 section - look for tables after the subtitle
        let mut status_change_info = StatusChangeInfo::default();

        for element in document.select(&selector) {
            if element.inner_html().trim() == "異動情報" {
                // Find the next table after this subtitle
                let mut current = element.next_sibling();
                while let Some(node) = current {
                    if let Some(table_element) = scraper::ElementRef::wrap(node) {
                        if table_element.value().name() == "table"
                            || table_element.select(&row_selector).next().is_some()
                        {
                            // Found a table, parse its rows
                            for row in table_element.select(&row_selector) {
                                let th = row.select(&th_selector).next();
                                let td = row.select(&td_selector).next();

                                if let (Some(th), Some(td)) = (th, td) {
                                    let label = th.inner_html().trim().to_string();
                                    let value = td.inner_html().trim().to_string();

                                    if label == "学籍状況" {
                                        // Split by <BR> and process each status entry
                                        let statuses: Vec<String> = value
                                            .split("<BR>")
                                            .filter_map(|s| {
                                                let trimmed = s.trim();
                                                if trimmed.is_empty() {
                                                    None
                                                } else {
                                                    Some(trimmed.to_string())
                                                }
                                            })
                                            .collect();

                                        status_change_info.academic_status_history = statuses;
                                    }
                                }
                            }
                            break;
                        }
                    }
                    current = node.next_sibling();
                }
                break;
            }
        }

        Ok(status_change_info)
    }

    /// 日付形式を変換する（例：「2003年01月10日」→「2003-01-10」）
    fn format_date(&self, date_str: &str) -> Result<String, ParseError> {
        if date_str.is_empty() {
            return Ok(String::new());
        }

        // Remove any HTML tags and trim
        let clean_date = date_str.replace("<BR>", "").trim().to_string();

        // Convert Japanese date format to ISO format
        if let Some(captures) = regex_lite::Regex::new(r"(\d{4})年(\d{1,2})月(\d{1,2})日")
            .map_err(|e| ParseError::data_parsing_failed("date regex", &e.to_string()))?
            .captures(&clean_date)
        {
            let year = &captures[1];
            let month = format!("{:02}", captures[2].parse::<u32>().unwrap_or(1));
            let day = format!("{:02}", captures[3].parse::<u32>().unwrap_or(1));
            Ok(format!("{}-{}-{}", year, month, day))
        } else {
            Ok(clean_date)
        }
    }

    /// 年月形式を変換する（例：「2026年03月」→「2026-03」）
    fn format_year_month(&self, date_str: &str) -> Result<String, ParseError> {
        if date_str.is_empty() {
            return Ok(String::new());
        }

        // Convert Japanese year-month format to ISO format
        if let Some(captures) = regex_lite::Regex::new(r"(\d{4})年(\d{1,2})月")
            .map_err(|e| ParseError::data_parsing_failed("year-month regex", &e.to_string()))?
            .captures(date_str)
        {
            let year = &captures[1];
            let month = format!("{:02}", captures[2].parse::<u32>().unwrap_or(1));
            Ok(format!("{}-{}", year, month))
        } else {
            Ok(date_str.to_string())
        }
    }
}
