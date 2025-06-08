//! アンケート一覧パーサー実装
//!
//! HTMLドキュメントからアンケート一覧情報を解析するロジックを担当します。

use scraper::{Html, Selector};

use crate::utils::error::ParseError;

use super::model::{
    PaginationInfo, QuestionnaireItem, QuestionnaireLink, QuestionnaireList, ResponseStatus,
};

/// アンケート一覧パーサー実装
///
/// HTMLドキュメントからアンケート一覧の情報を抽出するパースロジックを提供します。
pub struct QuestionnaireListParserImpl;

impl QuestionnaireListParserImpl {
    /// 新しいパーサーインスタンスを作成
    ///
    /// # 戻り値
    ///
    /// 新しいパーサーインスタンス
    pub fn new() -> Self {
        Self
    }

    /// HTMLドキュメントからアンケート一覧を解析する
    ///
    /// # 引数
    ///
    /// * `document` - 解析対象のHTMLドキュメント
    ///
    /// # 戻り値
    ///
    /// 解析されたアンケート一覧情報
    ///
    /// # エラー
    ///
    /// HTMLの解析に失敗した場合は `ParseError` を返します。
    pub fn parse_document(&self, document: &Html) -> Result<QuestionnaireList, ParseError> {
        let questionnaires = self.parse_questionnaires(document)?;
        let pagination = self.parse_pagination(document)?;

        Ok(QuestionnaireList {
            questionnaires,
            pagination,
        })
    }

    /// アンケート一覧を解析する
    ///
    /// # 引数
    ///
    /// * `document` - 解析対象のHTMLドキュメント
    ///
    /// # 戻り値
    ///
    /// 解析されたアンケート項目のリスト
    ///
    /// # エラー
    ///
    /// テーブル構造の解析に失敗した場合は `ParseError` を返します。
    fn parse_questionnaires(&self, document: &Html) -> Result<Vec<QuestionnaireItem>, ParseError> {
        let table_selector = Selector::parse("table.listTable")
            .map_err(|e| ParseError::selector_creation_failed("table.listTable", &e.to_string()))?;

        let row_selector = Selector::parse("tbody tr")
            .map_err(|e| ParseError::selector_creation_failed("tbody tr", &e.to_string()))?;

        let cell_selector = Selector::parse("td")
            .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

        let mut questionnaires = Vec::new();

        if let Some(table) = document.select(&table_selector).next() {
            for row in table.select(&row_selector) {
                let cells: Vec<_> = row.select(&cell_selector).collect();

                if cells.len() >= 4 {
                    let title = cells[0].inner_html().trim().to_string();
                    let subject_html = cells[1].inner_html();
                    let subject_text = subject_html.trim();
                    let subject_name = {
                        if subject_text.is_empty() {
                            None
                        } else {
                            Some(subject_text.to_string())
                        }
                    };
                    let deadline = cells[2].inner_html().trim().to_string();
                    let status_html = cells[3].inner_html();
                    let status_text = status_html.trim();
                    let response_status = self.parse_response_status(status_text)?;

                    questionnaires.push(QuestionnaireItem {
                        title,
                        subject_name,
                        instructor_name: None, // 追加の列から抽出が必要
                        deadline,
                        response_status,
                        questionnaire_link: QuestionnaireLink::default(),
                    });
                }
            }
        }

        Ok(questionnaires)
    }

    /// 回答状況を解析する
    ///
    /// # 引数
    ///
    /// * `status_text` - 回答状況を表すテキスト
    ///
    /// # 戻り値
    ///
    /// 解析された回答状況
    ///
    /// # エラー
    ///
    /// 回答状況の解析に失敗した場合は `ParseError` を返します。
    fn parse_response_status(&self, status_text: &str) -> Result<ResponseStatus, ParseError> {
        match status_text {
            s if s.contains("未回答") => Ok(ResponseStatus::NotAnswered),
            s if s.contains("回答済") => Ok(ResponseStatus::Answered),
            s if s.contains("期限切れ") => Ok(ResponseStatus::Expired),
            _ => Ok(ResponseStatus::NotAnswered), // デフォルトは未回答
        }
    }

    /// ページネーション情報を解析する
    ///
    /// # 引数
    ///
    /// * `document` - 解析対象のHTMLドキュメント
    ///
    /// # 戻り値
    ///
    /// 解析されたページネーション情報（存在しない場合はNone）
    ///
    /// # エラー
    ///
    /// ページネーション構造の解析に失敗した場合は `ParseError` を返します。
    fn parse_pagination(&self, document: &Html) -> Result<Option<PaginationInfo>, ParseError> {
        // ページネーション要素を探す
        let pagination_selector = Selector::parse(".pagination, .pager, .page-nav, .page-list")
            .map_err(|e| {
                ParseError::selector_creation_failed(
                    ".pagination, .pager, .page-nav, .page-list",
                    &e.to_string(),
                )
            })?;

        if let Some(pagination_element) = document.select(&pagination_selector).next() {
            let mut current_page = 1;

            // ページ番号リンクを探す
            let link_selector = Selector::parse("a, span")
                .map_err(|e| ParseError::selector_creation_failed("a, span", &e.to_string()))?;

            let mut page_numbers = Vec::new();

            for element in pagination_element.select(&link_selector) {
                let element_html = element.inner_html();
                let text = element_html.trim();

                // 数字のページ番号を収集
                if let Ok(page_num) = text.parse::<u32>() {
                    page_numbers.push(page_num);

                    // 現在のページを特定（リンクでない要素や、activeクラスなど）
                    if element.value().name() == "span"
                        || element
                            .value()
                            .attr("class")
                            .unwrap_or("")
                            .contains("active")
                        || element
                            .value()
                            .attr("class")
                            .unwrap_or("")
                            .contains("current")
                    {
                        current_page = page_num;
                    }
                }
            }

            if !page_numbers.is_empty() {
                let total_pages = *page_numbers.iter().max().unwrap_or(&1);

                let pagination_info = PaginationInfo {
                    current_page,
                    total_pages,
                    total_count: 0, // コンテンツから抽出が必要
                    next_page_link: if current_page < total_pages {
                        Some("#".to_string())
                    } else {
                        None
                    },
                    previous_page_link: if current_page > 1 {
                        Some("#".to_string())
                    } else {
                        None
                    },
                };

                return Ok(Some(pagination_info));
            }
        }

        // フォーム内の隠しフィールドからページネーション情報を探す
        self.parse_pagination_from_form(document)
    }

    /// フォームの隠しフィールドからページネーション情報を取得する
    ///
    /// # 引数
    ///
    /// * `document` - 解析対象のHTMLドキュメント
    ///
    /// # 戻り値
    ///
    /// 解析されたページネーション情報（存在しない場合はNone）
    ///
    /// # エラー
    ///
    /// フォーム構造の解析に失敗した場合は `ParseError` を返します。
    fn parse_pagination_from_form(
        &self,
        document: &Html,
    ) -> Result<Option<PaginationInfo>, ParseError> {
        let hidden_selector = Selector::parse("input[type='hidden']").map_err(|e| {
            ParseError::selector_creation_failed("input[type='hidden']", &e.to_string())
        })?;

        let mut current_page = 1;
        let mut total_pages = 1;

        for hidden in document.select(&hidden_selector) {
            if let Some(name) = hidden.value().attr("name") {
                if let Some(value) = hidden.value().attr("value") {
                    match name {
                        n if n.contains("currentPage") || n.contains("pageNo") => {
                            current_page = value.parse::<u32>().unwrap_or(1);
                        }
                        n if n.contains("totalPages") || n.contains("maxPage") => {
                            total_pages = value.parse::<u32>().unwrap_or(1);
                        }
                        _ => {}
                    }
                }
            }
        }

        if total_pages > 1 || current_page > 1 {
            let pagination_info = PaginationInfo {
                current_page,
                total_pages,
                total_count: 0, // コンテンツから抽出が必要
                next_page_link: if current_page < total_pages {
                    Some("#".to_string())
                } else {
                    None
                },
                previous_page_link: if current_page > 1 {
                    Some("#".to_string())
                } else {
                    None
                },
            };
            Ok(Some(pagination_info))
        } else {
            Ok(None)
        }
    }
}
