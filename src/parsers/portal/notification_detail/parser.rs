//! お知らせ詳細のパーサー実装
//!
//! UNIVERSAL PASSPORT EXのお知らせ詳細ポップアップ画面のHTML解析ロジックを提供します。
//! タイトル、送信者、本文、添付ファイルなどの詳細情報を解析します。

use scraper::{Html, Selector};

use crate::common::traits::PageParser;
use crate::utils::error::ParseError;
use super::model::{NotificationDetail, AttachmentFile};

/// お知らせ詳細パーサー
pub struct NotificationDetailParser;

impl NotificationDetailParser {
    /// 新しいパーサーインスタンスを作成
    pub fn new() -> Result<Self, ParseError> {
        Ok(Self)
    }

    /// タイトル情報を解析する
    fn parse_title(&self, document: &Html) -> Result<String, ParseError> {
        // タイトル要素を探す（複数のセレクターを試行）
        let title_selectors = vec![
            ".popup-title",
            ".notification-title", 
            "h1",
            "h2",
            ".title",
            "#title"
        ];

        for selector_str in title_selectors {
            let selector = Selector::parse(selector_str)
                .map_err(|e| ParseError::selector_creation_failed(selector_str, &e.to_string()))?;

            if let Some(title_elem) = document.select(&selector).next() {
                let title_text = title_elem.inner_html().trim().to_string();
                if !title_text.is_empty() && title_text != "&nbsp;" {
                    return Ok(title_text);
                }
            }
        }

        Ok("".to_string())
    }

    /// 送信者情報を解析する
    fn parse_from(&self, document: &Html) -> Result<String, ParseError> {
        // 送信者要素を探す（複数のセレクターを試行）
        let from_selectors = vec![
            ".sender",
            ".from",
            ".author",
            "span:contains('送信者')",
            "td:contains('送信者')"
        ];

        for selector_str in from_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(from_elem) = document.select(&selector).next() {
                    let from_text = from_elem.inner_html().trim().to_string();
                    if !from_text.is_empty() && from_text != "&nbsp;" {
                        // "送信者:" プレフィックスがある場合は除去
                        return Ok(from_text.replace("送信者:", "").replace("送信者：", "").trim().to_string());
                    }
                }
            }
        }

        // テーブル構造からの抽出を試行
        let table_selector = Selector::parse("table")
            .map_err(|e| ParseError::selector_creation_failed("table", &e.to_string()))?;

        for table in document.select(&table_selector) {
            let row_selector = Selector::parse("tr")
                .map_err(|e| ParseError::selector_creation_failed("tr", &e.to_string()))?;

            for row in table.select(&row_selector) {
                let cell_selector = Selector::parse("td")
                    .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

                let cells: Vec<_> = row.select(&cell_selector).collect();
                if cells.len() >= 2 {
                    let header_text = cells[0].inner_html();
                    let header = header_text.trim();
                    if header.contains("送信者") || header.contains("From") {
                        return Ok(cells[1].inner_html().trim().to_string());
                    }
                }
            }
        }

        Ok("".to_string())
    }

    /// メイン本文を解析する
    fn parse_main_text(&self, document: &Html) -> Result<String, ParseError> {
        // 本文要素を探す（複数のセレクターを試行）
        let text_selectors = vec![
            ".main-content",
            ".content",
            ".message",
            ".body",
            ".text-content",
            "p"
        ];

        for selector_str in text_selectors {
            let selector = Selector::parse(selector_str)
                .map_err(|e| ParseError::selector_creation_failed(selector_str, &e.to_string()))?;

            let mut content_parts = Vec::new();
            for text_elem in document.select(&selector) {
                let text_content = text_elem.inner_html().trim().to_string();
                if !text_content.is_empty() && text_content != "&nbsp;" {
                    content_parts.push(text_content);
                }
            }

            if !content_parts.is_empty() {
                return Ok(content_parts.join("\n"));
            }
        }

        // 全体のテキストコンテンツから抽出（最後の手段）
        let body_selector = Selector::parse("body")
            .map_err(|e| ParseError::selector_creation_failed("body", &e.to_string()))?;

        if let Some(body) = document.select(&body_selector).next() {
            let body_text = body.inner_html();
            // HTMLタグを除去して純粋なテキストを抽出
            let text_only = body_text
                .replace("<br>", "\n")
                .replace("<br/>", "\n")
                .replace("<p>", "\n")
                .replace("</p>", "\n");
            return Ok(text_only);
        }

        Ok("".to_string())
    }

    /// 添付ファイル情報を解析する
    fn parse_attachments(&self, document: &Html) -> Result<Vec<AttachmentFile>, ParseError> {
        let mut attachments = Vec::new();

        // 添付ファイルテーブルを探す
        let attachment_selectors = vec![
            ".attachment-table",
            "table:contains('添付')",
            ".file-table"
        ];

        for selector_str in attachment_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for table in document.select(&selector) {
                    let row_selector = Selector::parse("tr")
                        .map_err(|e| ParseError::selector_creation_failed("tr", &e.to_string()))?;

                    for row in table.select(&row_selector) {
                        let cell_selector = Selector::parse("td")
                            .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

                        let cells: Vec<_> = row.select(&cell_selector).collect();
                        
                        if cells.len() >= 2 {
                            let mut attachment = AttachmentFile::default();

                            // ファイル名を取得
                            attachment.file_name = cells[0].inner_html().trim().to_string();

                            // ファイルサイズを取得
                            if cells.len() > 1 {
                                attachment.file_size = cells[1].inner_html().trim().to_string();
                            }

                            // ダウンロードボタンIDを取得
                            let button_selector = Selector::parse("input[type='button'], button")
                                .map_err(|e| ParseError::selector_creation_failed("input[type='button'], button", &e.to_string()))?;

                            for button in row.select(&button_selector) {
                                if let Some(id) = button.value().attr("id") {
                                    attachment.download_button_id = id.to_string();
                                    break;
                                } else if let Some(name) = button.value().attr("name") {
                                    attachment.download_button_id = name.to_string();
                                    break;
                                }
                            }

                            if !attachment.file_name.is_empty() {
                                attachments.push(attachment);
                            }
                        }
                    }
                }
            }
        }

        // ダウンロードリンクからの抽出（フォールバック）
        let download_link_selector = Selector::parse("a[href*='download'], a:contains('ダウンロード')")
            .map_err(|e| ParseError::selector_creation_failed("a[href*='download'], a:contains('ダウンロード')", &e.to_string()))?;

        for link in document.select(&download_link_selector) {
            let mut attachment = AttachmentFile::default();
            attachment.file_name = link.inner_html().trim().to_string();

            if let Some(href) = link.value().attr("href") {
                attachment.download_button_id = href.to_string();
            }

            if !attachment.file_name.is_empty() {
                attachments.push(attachment);
            }
        }

        Ok(attachments)
    }

    /// 閉じるボタン情報を解析する
    fn parse_close_button(&self, document: &Html) -> Result<String, ParseError> {
        // 閉じるボタンを探す
        let close_selectors = vec![
            "input[value*='閉じる']",
            "button:contains('閉じる')",
            "input[value*='Close']",
            "button:contains('Close')",
            ".close-button"
        ];

        for selector_str in close_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(close_elem) = document.select(&selector).next() {
                    if let Some(value) = close_elem.value().attr("value") {
                        return Ok(value.to_string());
                    } else {
                        return Ok(close_elem.inner_html().trim().to_string());
                    }
                }
            }
        }

        Ok("閉じる".to_string()) // デフォルト値
    }
}

impl PageParser<NotificationDetail> for NotificationDetailParser {
    const PAGE_TYPE: &'static str = "お知らせ詳細";

    fn parse_document(&self, document: &Html) -> Result<NotificationDetail, ParseError> {
        let title = self.parse_title(document)?;
        let from = self.parse_from(document)?;
        let main_text = self.parse_main_text(document)?;
        let attachments = self.parse_attachments(document)?;
        let close_button = self.parse_close_button(document)?;

        Ok(NotificationDetail {
            title,
            from,
            main_text,
            attachments,
            close_button,
        })
    }
}