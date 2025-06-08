//! 成績照会パーサー実装
//!
//! HTMLドキュメントから成績照会データを解析する機能を提供します。

use regex_lite;
use scraper::{Html, Selector};

use crate::utils::error::ParseError;

use super::model::{
    CategoryCredits, CreditDetails, CreditSummary, DisplayPattern, DisplaySettings, Grade,
    GradeInquiry, Semester, Subject, SubjectCategory,
};

/// 成績照会パーサー実装
pub struct GradeInquiryParserImpl {
    display_pattern: DisplayPattern,
    display_settings: DisplaySettings,
}

impl GradeInquiryParserImpl {
    /// 新しいパーサーインスタンスを作成
    pub fn new(display_pattern: DisplayPattern, display_settings: DisplaySettings) -> Self {
        Self {
            display_pattern,
            display_settings,
        }
    }

    /// HTMLドキュメントから成績照会を解析する
    pub fn parse_document(&self, document: &Html) -> Result<GradeInquiry, ParseError> {
        let subjects = self.parse_subjects(document)?;
        let gpa_score = self.parse_gpa(document)?;
        let credit_summary = self.parse_credit_summary(document)?;

        Ok(GradeInquiry {
            display_pattern: self.display_pattern.clone(),
            display_settings: self.display_settings.clone(),
            subjects,
            gpa_score,
            credit_summary,
        })
    }

    /// 科目一覧を解析する
    fn parse_subjects(&self, document: &Html) -> Result<Vec<Subject>, ParseError> {
        let table_selector = Selector::parse("table.listTable")
            .map_err(|e| ParseError::selector_creation_failed("table.listTable", &e.to_string()))?;

        let row_selector = Selector::parse("tbody tr")
            .map_err(|e| ParseError::selector_creation_failed("tbody tr", &e.to_string()))?;

        let cell_selector = Selector::parse("td")
            .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

        let mut subjects = Vec::new();

        if let Some(table) = document.select(&table_selector).next() {
            for row in table.select(&row_selector) {
                let cells: Vec<_> = row.select(&cell_selector).collect();

                if cells.len() >= 5 {
                    let name = cells[0].inner_html().trim().to_string();
                    let credit_count = cells[1].inner_html().trim().parse::<u32>().ok();
                    let grade_html = cells[2].inner_html();
                    let grade_text = grade_html.trim();
                    let grade = self.parse_grade(grade_text)?;
                    let instructor_name = cells[3].inner_html().trim().to_string();

                    subjects.push(Subject {
                        name,
                        credit_count,
                        grade: Some(grade),
                        numeric_score: None, // Would need additional parsing
                        academic_year: 2025, // Would need to be extracted
                        semester: Semester::Spring, // Would need to be extracted
                        instructor_name,
                        is_currently_enrolled: false, // Would need to be determined
                        category: SubjectCategory::default(),
                    });
                }
            }
        }

        Ok(subjects)
    }

    /// 成績評価を解析する
    fn parse_grade(&self, grade_text: &str) -> Result<Grade, ParseError> {
        match grade_text {
            "秀" => Ok(Grade::AA),
            "優" => Ok(Grade::A),
            "良" => Ok(Grade::B),
            "可" => Ok(Grade::C),
            "不可" => Ok(Grade::D),
            "合格" => Ok(Grade::Pass),
            _ => Ok(Grade::NoEvaluation),
        }
    }

    /// GPA得点を解析する
    fn parse_gpa(&self, document: &Html) -> Result<f64, ParseError> {
        // GPA表示エリアを探す
        let gpa_selector = Selector::parse(".gpaScore, .gpa, #gpaValue").map_err(|e| {
            ParseError::selector_creation_failed(".gpaScore, .gpa, #gpaValue", &e.to_string())
        })?;

        if let Some(gpa_element) = document.select(&gpa_selector).next() {
            let gpa_text = gpa_element.inner_html();
            let gpa_text_trimmed = gpa_text.trim();

            // GPAの数値部分を抽出（例：「GPA: 3.45」から「3.45」を抽出）
            let gpa_regex = regex_lite::Regex::new(r"(\d+\.\d+)")
                .map_err(|e| ParseError::data_parsing_failed("gpa regex", &e.to_string()))?;

            if let Some(captures) = gpa_regex.captures(gpa_text_trimmed) {
                return captures[1]
                    .parse::<f64>()
                    .map_err(|e| ParseError::data_parsing_failed("gpa parsing", &e.to_string()));
            }
        }

        // GPAの表示が見つからない場合は、成績表から計算
        self.calculate_gpa_from_grades(document)
    }

    /// 成績から GPA を計算する
    fn calculate_gpa_from_grades(&self, document: &Html) -> Result<f64, ParseError> {
        let table_selector = Selector::parse("table.listTable")
            .map_err(|e| ParseError::selector_creation_failed("table.listTable", &e.to_string()))?;

        let row_selector = Selector::parse("tbody tr")
            .map_err(|e| ParseError::selector_creation_failed("tbody tr", &e.to_string()))?;

        let cell_selector = Selector::parse("td")
            .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

        let mut total_points = 0.0;
        let mut total_credits = 0;

        if let Some(table) = document.select(&table_selector).next() {
            for row in table.select(&row_selector) {
                let cells: Vec<_> = row.select(&cell_selector).collect();

                if cells.len() >= 3 {
                    if let Ok(credits) = cells[1].inner_html().trim().parse::<u32>() {
                        let grade_html = cells[2].inner_html();
                        let grade_text = grade_html.trim();
                        let grade_point = self.grade_to_point(grade_text);

                        if grade_point > 0.0 {
                            total_points += grade_point * credits as f64;
                            total_credits += credits;
                        }
                    }
                }
            }
        }

        if total_credits > 0 {
            Ok(total_points / total_credits as f64)
        } else {
            Ok(0.0)
        }
    }

    /// 成績評価をGPAポイントに変換
    fn grade_to_point(&self, grade_text: &str) -> f64 {
        match grade_text {
            "秀" => 4.0,
            "優" => 3.0,
            "良" => 2.0,
            "可" => 1.0,
            "不可" => 0.0,
            "合格" => 3.0, // 合格は平均的な評価として扱う
            _ => 0.0,
        }
    }

    /// 単位取得状況を解析する
    fn parse_credit_summary(&self, document: &Html) -> Result<CreditSummary, ParseError> {
        let mut credit_summary = CreditSummary::default();

        // 単位状況テーブルを探す
        let summary_table_selector =
            Selector::parse("table.creditSummary, .creditTable, .unitTable").map_err(|e| {
                ParseError::selector_creation_failed(
                    "table.creditSummary, .creditTable, .unitTable",
                    &e.to_string(),
                )
            })?;

        if let Some(table) = document.select(&summary_table_selector).next() {
            let row_selector = Selector::parse("tr")
                .map_err(|e| ParseError::selector_creation_failed("tr", &e.to_string()))?;

            let cell_selector = Selector::parse("td, th")
                .map_err(|e| ParseError::selector_creation_failed("td, th", &e.to_string()))?;

            for row in table.select(&row_selector) {
                let cells: Vec<_> = row.select(&cell_selector).collect();

                if cells.len() >= 4 {
                    let category_name = cells[0].inner_html().trim().to_string();

                    // 数値セルをパース
                    let required = cells[1].inner_html().trim().parse::<u32>().unwrap_or(0);
                    let completed = cells[2].inner_html().trim().parse::<u32>().unwrap_or(0);
                    let current = cells[3].inner_html().trim().parse::<u32>().unwrap_or(0);

                    let credit_details = CreditDetails {
                        required_for_graduation: required,
                        completed_credits: completed,
                        currently_enrolled_credits: current,
                        total_credits: completed + current,
                    };

                    match category_name.as_str() {
                        s if s.contains("全体") || s.contains("合計") => {
                            credit_summary.overall = credit_details;
                        }
                        s if s.contains("共通") || s.contains("教養") => {
                            credit_summary.common_education = credit_details;
                        }
                        s if s.contains("専門") => {
                            credit_summary.specialized_education = credit_details;
                        }
                        _ => {
                            credit_summary.category_breakdown.push(CategoryCredits {
                                category_name,
                                credit_details,
                            });
                        }
                    }
                }
            }
        } else {
            // テーブルが見つからない場合は科目一覧から計算
            self.calculate_credit_summary_from_subjects(document, &mut credit_summary)?;
        }

        Ok(credit_summary)
    }

    /// 科目一覧から単位状況を計算する
    fn calculate_credit_summary_from_subjects(
        &self,
        document: &Html,
        credit_summary: &mut CreditSummary,
    ) -> Result<(), ParseError> {
        let table_selector = Selector::parse("table.listTable")
            .map_err(|e| ParseError::selector_creation_failed("table.listTable", &e.to_string()))?;

        let row_selector = Selector::parse("tbody tr")
            .map_err(|e| ParseError::selector_creation_failed("tbody tr", &e.to_string()))?;

        let cell_selector = Selector::parse("td")
            .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

        let mut completed_credits = 0;
        let mut current_credits = 0;

        if let Some(table) = document.select(&table_selector).next() {
            for row in table.select(&row_selector) {
                let cells: Vec<_> = row.select(&cell_selector).collect();

                if cells.len() >= 3 {
                    if let Ok(credits) = cells[1].inner_html().trim().parse::<u32>() {
                        let grade_html = cells[2].inner_html();
                        let grade_text = grade_html.trim();

                        if grade_text.is_empty() || grade_text == "-" {
                            current_credits += credits;
                        } else if !grade_text.contains("不可") {
                            completed_credits += credits;
                        }
                    }
                }
            }
        }

        // 全体の単位状況を設定
        credit_summary.overall = CreditDetails {
            required_for_graduation: 124, // 一般的な卒業要件単位数
            completed_credits,
            currently_enrolled_credits: current_credits,
            total_credits: completed_credits + current_credits,
        };

        Ok(())
    }
}
