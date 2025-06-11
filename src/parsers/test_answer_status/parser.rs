//! テスト解答状況パーサーの実装
//!
//! UNIVERSAL PASSPORT EXのテスト解答一覧ページ（Stb00101A）のHTMLを解析し、
//! 構造化されたデータとして抽出する機能を提供します。

use regex_lite::Regex;
use scraper::{Html, Selector};

use super::model::*;
use crate::common::traits::PageParser;
use crate::utils::error::ParseError;

/// テスト解答状況パーサーの実装
#[derive(Debug)]
pub struct TestAnswerStatusParserImpl {
    /// デバッグモード
    debug_mode: bool,
    /// 厳密なパースモード
    strict_mode: bool,
}

impl TestAnswerStatusParserImpl {
    /// デフォルト設定で新しいパーサーインスタンスを作成
    pub fn new() -> Result<Self, ParseError> {
        Ok(Self {
            debug_mode: false,
            strict_mode: false,
        })
    }

    /// 設定を指定して新しいパーサーインスタンスを作成
    pub fn new_with_config(debug_mode: bool, strict_mode: bool) -> Result<Self, ParseError> {
        Ok(Self {
            debug_mode,
            strict_mode,
        })
    }

    /// 年度情報を抽出
    fn parse_academic_year(&self, document: &Html) -> Result<String, ParseError> {
        let selector = Selector::parse("#form1\\:htmlNendo").map_err(|e| {
            ParseError::selector_creation_failed(
                "#form1:htmlNendo",
                &format!("CSS selector parsing failed: {}", e),
            )
        })?;

        if let Some(element) = document.select(&selector).next() {
            let text = element.text().collect::<String>();
            // HTMLエンティティをデコード（例: &#24180;&#24230; -> 年度）
            let decoded = self.decode_html_entities(&text);
            Ok(decoded)
        } else {
            if self.strict_mode {
                return Err(ParseError::element_not_found(
                    "#form1:htmlNendo",
                    "Academic year element not found",
                ));
            }
            Ok("不明".to_string())
        }
    }

    /// タブ情報（各状態の件数）を抽出
    fn parse_tab_info(&self, document: &Html) -> Result<TabInfo, ParseError> {
        let mut tab_info = TabInfo::default();

        // 未実施タブの件数
        if let Some(count) =
            self.extract_tab_count(document, "form1:htmlTab1aCount", "form1:htmlTab1bCount")?
        {
            tab_info.not_implemented_count = count;
        }

        // 実施中タブの件数
        if let Some(count) =
            self.extract_tab_count(document, "form1:htmlTab2aCount", "form1:htmlTab2bCount")?
        {
            tab_info.in_progress_count = count;
        }

        // 実施済タブの件数
        if let Some(count) =
            self.extract_tab_count(document, "form1:htmlTab3aCount", "form1:htmlTab3bCount")?
        {
            tab_info.completed_count = count;
        }

        Ok(tab_info)
    }

    /// 特定のタブの件数を抽出
    fn extract_tab_count(
        &self,
        document: &Html,
        id_a: &str,
        id_b: &str,
    ) -> Result<Option<u32>, ParseError> {
        // アクティブタブ（_a）とリンクタブ（_b）の両方を試す
        let ids = vec![id_a.to_string(), id_b.to_string()];
        for id in &ids {
            let selector_str = format!("#{}", id.replace(":", "\\:"));
            let selector = match Selector::parse(&selector_str) {
                Ok(s) => s,
                Err(_) => continue,
            };

            if let Some(element) = document.select(&selector).next() {
                let text = element.text().collect::<String>();
                let decoded = self.decode_html_entities(&text);
                // (数字) の形式から数字を抽出
                if let Some(captures) = Regex::new(r"\((\d+)\)").unwrap().captures(&decoded) {
                    if let Some(count_str) = captures.get(1) {
                        if let Ok(count) = count_str.as_str().parse::<u32>() {
                            return Ok(Some(count));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    /// 現在のタブ状態を判定
    fn parse_current_tab(&self, document: &Html) -> Result<TestStatus, ParseError> {
        // アクティブなタブを探す（tab01_on.gif の背景画像を持つタブ）
        let selectors = [
            ("#form1\\:htmlTab1a", TestStatus::NotImplemented),
            ("#form1\\:htmlTab2a", TestStatus::InProgress),
            ("#form1\\:htmlTab3a", TestStatus::Completed),
        ];

        for (selector_str, status) in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if document.select(&selector).next().is_some() {
                    return Ok(status.clone());
                }
            }
        }

        // デフォルトは未実施
        Ok(TestStatus::NotImplemented)
    }

    /// テスト一覧を抽出
    fn parse_tests(
        &self,
        document: &Html,
        current_tab: &TestStatus,
    ) -> Result<Vec<TestItem>, ParseError> {
        match current_tab {
            TestStatus::NotImplemented => self.parse_not_implemented_tests(document),
            TestStatus::InProgress => self.parse_in_progress_tests(document),
            TestStatus::Completed => self.parse_completed_tests(document),
        }
    }

    /// 未実施テストの一覧を抽出
    fn parse_not_implemented_tests(&self, document: &Html) -> Result<Vec<TestItem>, ParseError> {
        let mut tests = Vec::new();

        // 未実施テストのテーブル（Stb00101B）を探す
        let table_selector =
            Selector::parse("#form1\\:Stb00101B\\:table1 tbody tr").map_err(|e| {
                ParseError::selector_creation_failed(
                    "#form1:Stb00101B:table1 tbody tr",
                    &format!("CSS selector parsing failed: {}", e),
                )
            })?;

        for row in document.select(&table_selector) {
            if let Ok(test) = self.parse_not_implemented_test_row(&row) {
                tests.push(test);
            } else if self.strict_mode {
                return Err(ParseError::data_extraction_failed(
                    "not implemented test row",
                    "Failed to parse test row data",
                ));
            }
        }

        Ok(tests)
    }

    /// 未実施テストの行をパース
    fn parse_not_implemented_test_row(
        &self,
        row: &scraper::ElementRef,
    ) -> Result<TestItem, ParseError> {
        let title = self.extract_text_by_class(row, "title")?;
        let course_name = self.extract_text_by_class(row, "jugyo")?;
        let start_date = self.extract_optional_text_by_class(row, "startDate");
        let end_date = self.extract_optional_text_by_class(row, "endDate");
        let time_limit = self.extract_time_limit(row)?;
        let can_reanswer = self.extract_reanswer_flag(row)?;

        let details = TestDetails::NotImplemented(NotImplementedDetails {
            start_date,
            end_date,
            time_limit_minutes: time_limit,
            can_reanswer,
        });

        Ok(TestItem {
            title: self.decode_html_entities(&title),
            course_name: self.decode_html_entities(&course_name),
            details,
        })
    }

    /// 実施中テストの一覧を抽出
    fn parse_in_progress_tests(&self, document: &Html) -> Result<Vec<TestItem>, ParseError> {
        let mut tests = Vec::new();

        // 実施中テストのテーブル（Stb00101C）を探す
        let table_selector =
            Selector::parse("#form1\\:Stb00101C\\:table1 tbody tr").map_err(|e| {
                ParseError::selector_creation_failed(
                    "#form1:Stb00101C:table1 tbody tr",
                    &format!("CSS selector parsing failed: {}", e),
                )
            })?;

        for row in document.select(&table_selector) {
            if let Ok(test) = self.parse_in_progress_test_row(&row) {
                tests.push(test);
            } else if self.strict_mode {
                return Err(ParseError::data_extraction_failed(
                    "in progress test row",
                    "Failed to parse test row data",
                ));
            }
        }

        Ok(tests)
    }

    /// 実施中テストの行をパース
    fn parse_in_progress_test_row(
        &self,
        row: &scraper::ElementRef,
    ) -> Result<TestItem, ParseError> {
        let title = self.extract_text_by_class(row, "title")?;
        let course_name = self.extract_text_by_class(row, "jugyo")?;
        let attempt_count = self.extract_attempt_count(row)?;
        let latest_attempt_date = self.extract_optional_text_by_class(row, "SaishinJissiDate");
        let duration_minutes = self.extract_duration_minutes(row)?;
        let score = self.extract_optional_text_by_class(row, "Score");

        let details = TestDetails::InProgress(InProgressDetails {
            attempt_count,
            latest_attempt_date: latest_attempt_date.map(|s| self.decode_html_entities(&s)),
            duration_minutes,
            score: score.map(|s| self.decode_html_entities(&s)),
        });

        Ok(TestItem {
            title: self.decode_html_entities(&title),
            course_name: self.decode_html_entities(&course_name),
            details,
        })
    }

    /// 実施済テストの一覧を抽出
    fn parse_completed_tests(&self, document: &Html) -> Result<Vec<TestItem>, ParseError> {
        let mut tests = Vec::new();

        // 実施済テストのテーブル（Stb00101D）を探す
        let table_selector =
            Selector::parse("#form1\\:Stb00101D\\:table1 tbody tr").map_err(|e| {
                ParseError::selector_creation_failed(
                    "#form1:Stb00101D:table1 tbody tr",
                    &format!("CSS selector parsing failed: {}", e),
                )
            })?;

        for row in document.select(&table_selector) {
            if let Ok(test) = self.parse_completed_test_row(&row) {
                tests.push(test);
            } else if self.strict_mode {
                return Err(ParseError::data_extraction_failed(
                    "completed test row",
                    "Failed to parse test row data",
                ));
            }
        }

        Ok(tests)
    }

    /// 実施済テストの行をパース
    fn parse_completed_test_row(&self, row: &scraper::ElementRef) -> Result<TestItem, ParseError> {
        let title = self.extract_text_by_class(row, "title")?;
        let course_name = self.extract_text_by_class(row, "jugyo")?;
        let attempt_count = self.extract_attempt_count(row)?;
        let latest_attempt_date = self.extract_optional_text_by_class(row, "SaishinJissiDate");
        let duration_minutes = self.extract_duration_minutes(row)?;
        let score = self.extract_optional_text_by_class(row, "Score");

        let details = TestDetails::Completed(CompletedDetails {
            attempt_count,
            latest_attempt_date: latest_attempt_date.map(|s| self.decode_html_entities(&s)),
            duration_minutes,
            score: score.map(|s| self.decode_html_entities(&s)),
        });

        Ok(TestItem {
            title: self.decode_html_entities(&title),
            course_name: self.decode_html_entities(&course_name),
            details,
        })
    }

    /// クラス名でテキストを抽出
    fn extract_text_by_class(
        &self,
        element: &scraper::ElementRef,
        class_name: &str,
    ) -> Result<String, ParseError> {
        let selector = Selector::parse(&format!(".{}", class_name)).map_err(|e| {
            ParseError::selector_creation_failed(
                &format!(".{}", class_name),
                &format!("CSS selector parsing failed: {}", e),
            )
        })?;

        if let Some(cell) = element.select(&selector).next() {
            Ok(cell.text().collect::<String>().trim().to_string())
        } else {
            Err(ParseError::element_not_found(
                &format!(".{}", class_name),
                &format!("Element with class '{}' not found", class_name),
            ))
        }
    }

    /// クラス名でオプショナルなテキストを抽出
    fn extract_optional_text_by_class(
        &self,
        element: &scraper::ElementRef,
        class_name: &str,
    ) -> Option<String> {
        if let Ok(text) = self.extract_text_by_class(element, class_name) {
            if text.trim().is_empty() {
                None
            } else {
                Some(text)
            }
        } else {
            None
        }
    }

    /// 制限時間を抽出
    fn extract_time_limit(&self, row: &scraper::ElementRef) -> Result<Option<u32>, ParseError> {
        if let Some(text) = self.extract_text_from_class_and_nested_output_span(row, "limitTime") {
            if let Ok(minutes) = text.parse::<u32>() {
                Ok(Some(minutes))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// 実施時間を抽出
    fn extract_duration_minutes(
        &self,
        row: &scraper::ElementRef,
    ) -> Result<Option<u32>, ParseError> {
        if let Some(text) = self.extract_text_from_class_and_nested_output_span(row, "JissiTime") {
            if let Ok(minutes) = text.parse::<u32>() {
                Ok(Some(minutes))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// 実施回数を抽出
    fn extract_attempt_count(&self, row: &scraper::ElementRef) -> Result<u32, ParseError> {
        let text = self
            .extract_text_from_class_and_nested_output_span(row, "jissiKaisu")
            .ok_or_else(|| {
                ParseError::data_extraction_failed("attempt count", "jissiKaisu element not found")
            })?;

        text.parse::<u32>()
            .map_err(|_| ParseError::data_parsing_failed("attempt count", &text))
    }

    /// 再解答フラグを抽出
    fn extract_reanswer_flag(&self, row: &scraper::ElementRef) -> Result<bool, ParseError> {
        if let Some(text) = self.extract_optional_text_by_class(row, "saikaito") {
            let decoded = self.decode_html_entities(&text);
            Ok(decoded.contains("あり") || decoded.contains("可"))
        } else {
            Ok(false)
        }
    }

    /// HTMLエンティティをデコード
    fn decode_html_entities(&self, text: &str) -> String {
        // 基本的なHTMLエンティティのデコード
        text.replace("&#24180;", "年")
            .replace("&#24230;", "度")
            .replace("&#26410;", "未")
            .replace("&#23455;", "実")
            .replace("&#26045;", "施")
            .replace("&#20013;", "中")
            .replace("&#28168;", "済")
            .replace("&#12486;", "テ")
            .replace("&#12473;", "ス")
            .replace("&#12488;", "ト")
            .replace("&#12479;", "タ")
            .replace("&#12452;", "イ")
            .replace("&#12523;", "ル")
            .replace("&#25480;", "授")
            .replace("&#26989;", "業")
            .replace("&#38283;", "開")
            .replace("&#22987;", "始")
            .replace("&#26085;", "日")
            .replace("&#32066;", "終")
            .replace("&#20102;", "了")
            .replace("&#21046;", "制")
            .replace("&#38480;", "限")
            .replace("&#26178;", "時")
            .replace("&#38291;", "間")
            .replace("&#20877;", "再")
            .replace("&#35299;", "解")
            .replace("&#31572;", "答")
            .replace("&#12354;", "あ")
            .replace("&#12426;", "り")
            .replace("&#22238;", "回")
            .replace("&#25968;", "数")
            .replace("&#26368;", "最")
            .replace("&#26032;", "新")
            .replace("&#20998;", "分")
            .replace("&#28857;", "点")
            .replace("&#65288;", "(")
            .replace("&#65289;", ")")
            .replace("&#22303;", "土")
            .replace("&#28779;", "火")
            .replace("&#65306;", ":")
            .replace("&nbsp;", " ")
    }

    /// 特定のクラスを持つ要素内の、outputTextクラスを持つスパンのテキストを抽出
    fn extract_text_from_class_and_nested_output_span(
        &self,
        element: &scraper::ElementRef,
        parent_class_name: &str,
    ) -> Option<String> {
        let parent_selector_str = format!(".{}", parent_class_name);
        let parent_selector = match Selector::parse(&parent_selector_str) {
            Ok(s) => s,
            Err(_) => return None,
        };

        if let Some(parent_element) = element.select(&parent_selector).next() {
            let nested_span_selector_str = ".outputText";
            let nested_span_selector = match Selector::parse(nested_span_selector_str) {
                Ok(s) => s,
                Err(_) => return None,
            };

            if let Some(span_element) = parent_element.select(&nested_span_selector).next() {
                let text = span_element.text().collect::<String>().trim().to_string();
                if text.is_empty() {
                    None
                } else {
                    Some(text)
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl PageParser<TestAnswerStatus> for TestAnswerStatusParserImpl {
    const PAGE_TYPE: &'static str = "Stb00101A";

    fn parse_document(&self, document: &Html) -> Result<TestAnswerStatus, ParseError> {
        let academic_year = self.parse_academic_year(document)?;
        let tab_info = self.parse_tab_info(document)?;
        let current_tab = self.parse_current_tab(document)?;
        let tests = self.parse_tests(document, &current_tab)?;

        Ok(TestAnswerStatus {
            academic_year,
            tab_info,
            current_tab,
            tests,
        })
    }
}
