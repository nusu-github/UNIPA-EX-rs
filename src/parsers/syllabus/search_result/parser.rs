//! シラバス検索結果のパーサー実装
//!
//! UNIVERSAL PASSPORT EXのシラバス検索結果ページから各種情報を抽出するパーサーロジックを提供します。

use super::model::*;
use crate::common::traits::PageParser;
use crate::utils::error::ParseError;
use scraper::{Html, Selector};

/// シラバス検索結果パーサー
pub struct SyllabusSearchResultParser;

impl SyllabusSearchResultParser {
    /// 新しいパーサーインスタンスを作成
    pub fn new() -> Result<Self, ParseError> {
        Ok(Self)
    }

    /// 検索条件を解析
    fn parse_search_conditions(&self, document: &Html) -> Result<SearchConditions, ParseError> {
        let selector = Selector::parse("#form1\\:htmlKensakuJyoken").map_err(|e| {
            ParseError::selector_creation_failed("#form1:htmlKensakuJyoken", &e.to_string())
        })?;

        let element = document.select(&selector).next().ok_or_else(|| {
            ParseError::element_not_found(
                "#form1:htmlKensakuJyoken",
                "検索条件要素が見つかりません",
            )
        })?;

        let text = element.inner_html();
        // <BR>と<br>の両方に対応
        let lines: Vec<&str> = text.split("<BR>").flat_map(|s| s.split("<br>")).collect();

        let mut academic_year_semester = String::new();
        let mut subject_name = None;
        let mut department_course = None;

        for line in lines {
            // 改行文字と余分なスペースを正規化
            let clean_line = line
                .replace("\n", "") // 改行文字を除去
                .replace("\r", "") // キャリッジリターンを除去
                .replace("　", " ") // 全角スペースを半角スペースに変換
                .split_whitespace() // 複数の空白文字を単一スペースに正規化
                .collect::<Vec<&str>>()
                .join(" ");

            if clean_line.starts_with("開講年度／学期：") {
                academic_year_semester = clean_line
                    .replace("開講年度／学期：", "")
                    .trim()
                    .to_string();
            } else if clean_line.starts_with("科目名：") {
                let name = clean_line.replace("科目名：", "").trim().to_string();
                if !name.is_empty() {
                    subject_name = Some(name);
                }
            } else if clean_line.starts_with("学科・コース／専攻：") {
                let dept = clean_line
                    .replace("学科・コース／専攻：", "")
                    .trim()
                    .to_string();
                if !dept.is_empty() {
                    department_course = Some(dept);
                }
            }
        }

        Ok(SearchConditions {
            academic_year_semester,
            subject_name,
            department_course,
        })
    }

    /// 検索結果のメタ情報を解析
    fn parse_result_metadata(&self, document: &Html) -> Result<ResultMetadata, ParseError> {
        // 総件数を取得
        let count_selector =
            Selector::parse("#form1\\:htmlKekkatable\\:htmlGokeiKensu").map_err(|e| {
                ParseError::selector_creation_failed(
                    "#form1:htmlKekkatable:htmlGokeiKensu",
                    &e.to_string(),
                )
            })?;

        let count_element = document.select(&count_selector).next().ok_or_else(|| {
            ParseError::element_not_found(
                "#form1:htmlKekkatable:htmlGokeiKensu",
                "総件数要素が見つかりません",
            )
        })?;

        let count_text = count_element.inner_html();
        let total_count = count_text
            .replace("件", "")
            .trim()
            .parse::<u32>()
            .map_err(|_| ParseError::data_parsing_failed("総件数", &count_text))?;

        // ページ情報を取得
        let page_selector = Selector::parse("#form1\\:htmlKekkatable\\:deluxe1__pagerText")
            .map_err(|e| {
                ParseError::selector_creation_failed(
                    "#form1:htmlKekkatable:deluxe1__pagerText",
                    &e.to_string(),
                )
            })?;

        let page_element = document.select(&page_selector).next().ok_or_else(|| {
            ParseError::element_not_found(
                "#form1:htmlKekkatable:deluxe1__pagerText",
                "ページ情報要素が見つかりません",
            )
        })?;

        let page_text = page_element.inner_html();
        let parts: Vec<&str> = page_text.split('/').collect();

        let current_page = if parts.len() >= 2 {
            parts[0].trim().parse::<u32>().unwrap_or(1)
        } else {
            1
        };

        let total_pages = if parts.len() >= 2 {
            parts[1]
                .replace(" ページ", "")
                .trim()
                .parse::<u32>()
                .unwrap_or(1)
        } else {
            1
        };

        Ok(ResultMetadata {
            total_count,
            current_page,
            total_pages,
        })
    }

    /// 科目エントリ一覧を解析
    fn parse_course_entries(&self, document: &Html) -> Result<Vec<CourseEntry>, ParseError> {
        let row_selector =
            Selector::parse("#form1\\:htmlKekkatable tbody tr.rowClass1").map_err(|e| {
                ParseError::selector_creation_failed(
                    "#form1:htmlKekkatable tbody tr.rowClass1",
                    &e.to_string(),
                )
            })?;

        let mut entries = Vec::new();

        for row in document.select(&row_selector) {
            let entry = self.parse_course_entry(&row)?;
            // 空のエントリをフィルタリング（科目名が空の場合はスキップ）
            if !entry.course_code_and_name.trim().is_empty() {
                entries.push(entry);
            }
        }

        Ok(entries)
    }

    /// 個別の科目エントリを解析
    fn parse_course_entry(&self, row: &scraper::ElementRef) -> Result<CourseEntry, ParseError> {
        // 開講曜日
        let schedule_selector = Selector::parse("td.yobi span")
            .map_err(|e| ParseError::selector_creation_failed("td.yobi span", &e.to_string()))?;
        let schedule_day_period = row
            .select(&schedule_selector)
            .next()
            .map(|e| {
                e.inner_html()
                    .replace("<BR>", "")
                    .replace("<br>", "")
                    .trim()
                    .to_string()
            })
            .unwrap_or_default();

        // 科目名とリンク情報
        let link_selector = Selector::parse("td.kamokuName a")
            .map_err(|e| ParseError::selector_creation_failed("td.kamokuName a", &e.to_string()))?;
        let link_element = row.select(&link_selector).next();

        let (course_code_and_name, syllabus_link) = if let Some(link) = link_element {
            let course_name = link
                .select(&Selector::parse("span").unwrap())
                .next()
                .map(|e| e.inner_html())
                .unwrap_or_default();

            let link_id = link.value().attr("id").unwrap_or_default().to_string();
            let onclick_action = link.value().attr("onclick").unwrap_or_default().to_string();

            (
                course_name,
                SyllabusLinkInfo {
                    link_id,
                    onclick_action,
                    is_active: true,
                },
            )
        } else {
            (String::new(), SyllabusLinkInfo::default())
        };

        // 教員氏名
        let instructor_selector = Selector::parse("td.kyoin span")
            .map_err(|e| ParseError::selector_creation_failed("td.kyoin span", &e.to_string()))?;
        let instructor_names = row
            .select(&instructor_selector)
            .next()
            .map(|e| e.inner_html())
            .unwrap_or_default();

        // 開講区分
        let course_type_selector = Selector::parse("td.kubun span")
            .map_err(|e| ParseError::selector_creation_failed("td.kubun span", &e.to_string()))?;
        let course_type = row
            .select(&course_type_selector)
            .next()
            .map(|e| {
                e.inner_html()
                    .replace("<BR>", "")
                    .replace("<br>", "")
                    .trim()
                    .to_string()
            })
            .unwrap_or_default();

        // 学年
        let grade_selector = Selector::parse("td.gakunen span")
            .map_err(|e| ParseError::selector_creation_failed("td.gakunen span", &e.to_string()))?;
        let target_grade = row
            .select(&grade_selector)
            .next()
            .map(|e| e.inner_html().trim().to_string())
            .filter(|s| !s.is_empty() && s != "　");

        // 開講学期
        let semester_selector = Selector::parse("td.gakki span")
            .map_err(|e| ParseError::selector_creation_failed("td.gakki span", &e.to_string()))?;
        let semester = row
            .select(&semester_selector)
            .next()
            .map(|e| {
                e.inner_html()
                    .replace("<BR>", "")
                    .replace("<br>", "")
                    .trim()
                    .to_string()
            })
            .unwrap_or_default();

        // 単位数
        let credits_selector = Selector::parse("td.tani span")
            .map_err(|e| ParseError::selector_creation_failed("td.tani span", &e.to_string()))?;
        let credits = row
            .select(&credits_selector)
            .next()
            .map(|e| e.inner_html())
            .unwrap_or_default();

        Ok(CourseEntry {
            schedule_day_period,
            course_code_and_name,
            instructor_names,
            course_type,
            target_grade,
            semester,
            credits,
            syllabus_link,
        })
    }

    /// ページネーション情報を解析
    fn parse_pagination(&self, document: &Html) -> Result<PaginationInfo, ParseError> {
        // ページ表示テキスト
        let page_text_selector = Selector::parse("#form1\\:htmlKekkatable\\:deluxe1__pagerText")
            .map_err(|e| {
                ParseError::selector_creation_failed(
                    "#form1:htmlKekkatable:deluxe1__pagerText",
                    &e.to_string(),
                )
            })?;
        let page_display_text = document
            .select(&page_text_selector)
            .next()
            .map(|e| e.inner_html())
            .unwrap_or_default();

        // 現在のページ番号
        let current_page_selector = Selector::parse("#form1\\:htmlKekkatable\\:web1 strong")
            .map_err(|e| {
                ParseError::selector_creation_failed(
                    "#form1:htmlKekkatable:web1 strong",
                    &e.to_string(),
                )
            })?;
        let current_page_number = document
            .select(&current_page_selector)
            .next()
            .map(|e| e.inner_html().parse::<u32>().unwrap_or(1))
            .unwrap_or(1);

        // ボタンの状態を確認
        let first_button_selector =
            Selector::parse("#form1\\:htmlKekkatable\\:deluxe1__pagerFirst").map_err(|e| {
                ParseError::selector_creation_failed(
                    "#form1:htmlKekkatable:deluxe1__pagerFirst",
                    &e.to_string(),
                )
            })?;
        let first_button_enabled = document
            .select(&first_button_selector)
            .next()
            .map(|e| !e.value().attr("disabled").is_some())
            .unwrap_or(false);

        let previous_button_selector =
            Selector::parse("#form1\\:htmlKekkatable\\:deluxe1__pagerPrevious").map_err(|e| {
                ParseError::selector_creation_failed(
                    "#form1:htmlKekkatable:deluxe1__pagerPrevious",
                    &e.to_string(),
                )
            })?;
        let previous_button_enabled = document
            .select(&previous_button_selector)
            .next()
            .map(|e| !e.value().attr("disabled").is_some())
            .unwrap_or(false);

        let next_button_selector = Selector::parse("#form1\\:htmlKekkatable\\:deluxe1__pagerNext")
            .map_err(|e| {
                ParseError::selector_creation_failed(
                    "#form1:htmlKekkatable:deluxe1__pagerNext",
                    &e.to_string(),
                )
            })?;
        let next_button_enabled = document
            .select(&next_button_selector)
            .next()
            .map(|e| !e.value().attr("disabled").is_some())
            .unwrap_or(false);

        let last_button_selector = Selector::parse("#form1\\:htmlKekkatable\\:deluxe1__pagerLast")
            .map_err(|e| {
                ParseError::selector_creation_failed(
                    "#form1:htmlKekkatable:deluxe1__pagerLast",
                    &e.to_string(),
                )
            })?;
        let last_button_enabled = document
            .select(&last_button_selector)
            .next()
            .map(|e| !e.value().attr("disabled").is_some())
            .unwrap_or(false);

        Ok(PaginationInfo {
            first_button_enabled,
            previous_button_enabled,
            next_button_enabled,
            last_button_enabled,
            page_display_text,
            current_page_number,
        })
    }

    /// フォーム情報を解析
    fn parse_form_info(&self, document: &Html) -> Result<FormInfo, ParseError> {
        let form_selector = Selector::parse("#form1")
            .map_err(|e| ParseError::selector_creation_failed("#form1", &e.to_string()))?;

        let form_element = document.select(&form_selector).next().ok_or_else(|| {
            ParseError::element_not_found("#form1", "フォーム要素が見つかりません")
        })?;

        let form_action = form_element
            .value()
            .attr("action")
            .unwrap_or_default()
            .to_string();

        let form_method = form_element
            .value()
            .attr("method")
            .unwrap_or_default()
            .to_string();

        let form_enctype = form_element
            .value()
            .attr("enctype")
            .unwrap_or_default()
            .to_string();

        // 隠しフィールドを取得
        let hidden_selector = Selector::parse("input[type=\"hidden\"]").map_err(|e| {
            ParseError::selector_creation_failed("input[type=\"hidden\"]", &e.to_string())
        })?;

        let mut hidden_fields = Vec::new();
        for input in form_element.select(&hidden_selector) {
            let field_name = input.value().attr("name").unwrap_or_default().to_string();
            let field_value = input.value().attr("value").unwrap_or_default().to_string();

            if !field_name.is_empty() {
                hidden_fields.push(HiddenField {
                    field_name,
                    field_value,
                });
            }
        }

        Ok(FormInfo {
            form_action,
            form_method,
            form_enctype,
            hidden_fields,
        })
    }
}

impl PageParser<SyllabusSearchResultPage> for SyllabusSearchResultParser {
    const PAGE_TYPE: &'static str = "シラバス検索結果";

    fn parse_document(&self, document: &Html) -> Result<SyllabusSearchResultPage, ParseError> {
        let search_conditions = self.parse_search_conditions(document)?;
        let result_metadata = self.parse_result_metadata(document)?;
        let course_entries = self.parse_course_entries(document)?;
        let pagination = self.parse_pagination(document)?;
        let form_info = self.parse_form_info(document)?;

        Ok(SyllabusSearchResultPage {
            search_conditions,
            result_metadata,
            course_entries,
            pagination,
            form_info,
        })
    }
}
