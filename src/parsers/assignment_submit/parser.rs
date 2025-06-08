//! 課題提出一覧パーサー実装
//!
//! HTMLドキュメントから課題提出一覧情報を抽出するパースロジックを提供します。

use scraper::{Html, Selector};

use super::model::{Assignment, AssignmentList, PaginationInfo, SubmissionStatus};
use crate::utils::error::ParseError;

/// 課題提出一覧パーサー実装
pub struct AssignmentListParserImpl {
    /// デバッグモード
    debug_mode: bool,
    /// 厳密なパースモード
    strict_mode: bool,
}

impl AssignmentListParserImpl {
    /// 新しいパーサーインスタンスを作成
    pub fn new_with_config(debug_mode: bool, strict_mode: bool) -> Self {
        Self {
            debug_mode,
            strict_mode,
        }
    }

    /// HTMLドキュメントから課題提出一覧を解析する
    pub fn parse_document(&self, document: &Html) -> Result<AssignmentList, ParseError> {
        let assignments = self.parse_assignments(document)?;
        let pagination = self.parse_pagination(document)?;

        Ok(AssignmentList {
            assignments,
            pagination,
        })
    }

    /// 課題一覧を解析する
    fn parse_assignments(&self, document: &Html) -> Result<Vec<Assignment>, ParseError> {
        let table_selector = Selector::parse("table.listTable")
            .map_err(|e| ParseError::selector_creation_failed("table.listTable", &e.to_string()))?;

        let row_selector = Selector::parse("tbody tr")
            .map_err(|e| ParseError::selector_creation_failed("tbody tr", &e.to_string()))?;

        let cell_selector = Selector::parse("td")
            .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

        let mut assignments = Vec::new();

        if let Some(table) = document.select(&table_selector).next() {
            for row in table.select(&row_selector) {
                let cells: Vec<_> = row.select(&cell_selector).collect();

                if cells.len() >= 4 {
                    let subject_name = cells[0].inner_html().trim().to_string();
                    let assignment_title = cells[1].inner_html().trim().to_string();
                    let due_date = self.format_due_date(cells[2].inner_html().trim())?;
                    let status_html = cells[3].inner_html();
                    let status_text = status_html.trim();
                    let submission_status = self.parse_submission_status(status_text)?;

                    assignments.push(Assignment {
                        subject_name,
                        assignment_title,
                        due_date,
                        submission_status,
                        description: None, // Would need to be extracted from detail pages
                        has_attachment: false, // Would need to be determined from status
                        submitted_file_name: None,
                        submission_date: None,
                        teacher_comment: None,
                        score: None,
                    });
                }
            }
        }

        Ok(assignments)
    }

    /// 提出期限を標準形式に変換する
    fn format_due_date(&self, date_str: &str) -> Result<String, ParseError> {
        if date_str.is_empty() {
            return Ok(String::new());
        }

        // Convert Japanese date format to ISO format
        if let Some(captures) =
            regex_lite::Regex::new(r"(\d{4})年(\d{1,2})月(\d{1,2})日\s*(\d{1,2}):(\d{2})")
                .map_err(|e| ParseError::data_parsing_failed("date regex", &e.to_string()))?
                .captures(date_str)
        {
            let year = &captures[1];
            let month = format!("{:02}", captures[2].parse::<u32>().unwrap_or(1));
            let day = format!("{:02}", captures[3].parse::<u32>().unwrap_or(1));
            let hour = format!("{:02}", captures[4].parse::<u32>().unwrap_or(0));
            let minute = format!("{:02}", captures[5].parse::<u32>().unwrap_or(0));
            Ok(format!("{}-{}-{} {}:{}", year, month, day, hour, minute))
        } else {
            Ok(date_str.to_string())
        }
    }

    /// 提出状況を解析する
    fn parse_submission_status(&self, status_text: &str) -> Result<SubmissionStatus, ParseError> {
        match status_text {
            s if s.contains("未提出") => Ok(SubmissionStatus::NotSubmitted),
            s if s.contains("提出済") => Ok(SubmissionStatus::Submitted),
            s if s.contains("期限切れ") => Ok(SubmissionStatus::Overdue),
            s if s.contains("評価済") => Ok(SubmissionStatus::Evaluated),
            _ => Ok(SubmissionStatus::NotSubmitted), // Default to not submitted
        }
    }

    /// ページネーション情報を解析する
    fn parse_pagination(&self, _document: &Html) -> Result<Option<PaginationInfo>, ParseError> {
        // ページネーション要素を探す
        let pagination_selector =
            Selector::parse(".pagination, .pager, .page-nav").map_err(|e| {
                ParseError::selector_creation_failed(
                    ".pagination, .pager, .page-nav",
                    &e.to_string(),
                )
            })?;

        if let Some(pagination_element) = _document.select(&pagination_selector).next() {
            let mut pagination_info = PaginationInfo::default();

            // ページ番号リンクを探す
            let link_selector = Selector::parse("a")
                .map_err(|e| ParseError::selector_creation_failed("a", &e.to_string()))?;

            let mut page_numbers = Vec::new();
            for link in pagination_element.select(&link_selector) {
                if let Ok(text) = link.inner_html().trim().parse::<u32>() {
                    page_numbers.push(text);
                }
            }

            // 現在のページ番号を特定（active class などから）
            let active_selector = Selector::parse(".active, .current").map_err(|e| {
                ParseError::selector_creation_failed(".active, .current", &e.to_string())
            })?;

            if let Some(active_element) = pagination_element.select(&active_selector).next() {
                if let Ok(current) = active_element.inner_html().trim().parse::<u32>() {
                    pagination_info.current_page = current;
                }
            } else if !page_numbers.is_empty() {
                pagination_info.current_page = 1; // デフォルト
            }

            if !page_numbers.is_empty() {
                pagination_info.total_pages = *page_numbers.iter().max().unwrap_or(&1);
                pagination_info.has_previous = pagination_info.current_page > 1;
                pagination_info.has_next =
                    pagination_info.current_page < pagination_info.total_pages;

                return Ok(Some(pagination_info));
            }
        }

        // ページネーションが見つからない場合は、総件数などから推測
        self.parse_pagination_from_content(_document)
    }

    /// コンテンツから間接的にページネーション情報を推測する
    fn parse_pagination_from_content(
        &self,
        _document: &Html,
    ) -> Result<Option<PaginationInfo>, ParseError> {
        // 結果件数表示を探す（例：「1-20件 / 全50件」）
        let count_info_selector =
            Selector::parse(".result-count, .count-info, .total").map_err(|e| {
                ParseError::selector_creation_failed(
                    ".result-count, .count-info, .total",
                    &e.to_string(),
                )
            })?;

        if let Some(count_element) = _document.select(&count_info_selector).next() {
            let count_text = count_element.inner_html();

            // 件数情報から総数を抽出
            let total_regex = regex_lite::Regex::new(r"全\s*(\d+)\s*件").map_err(|e| {
                ParseError::data_parsing_failed("total count regex", &e.to_string())
            })?;

            if let Some(captures) = total_regex.captures(&count_text) {
                if let Ok(total) = captures[1].parse::<u32>() {
                    let items_per_page = 20; // 一般的な1ページあたりの件数
                    let total_pages = total.div_ceil(items_per_page);

                    let mut pagination_info = PaginationInfo::default();
                    pagination_info.current_page = 1;
                    pagination_info.total_pages = total_pages;
                    pagination_info.has_previous = false;
                    pagination_info.has_next = total_pages > 1;

                    return Ok(Some(pagination_info));
                }
            }
        }

        Ok(None)
    }
}
