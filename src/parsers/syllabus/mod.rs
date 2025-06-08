//! シラバス関連のページパーサー
//!
//! UNIVERSAL PASSPORT EXのシラバス機能に関連するページを解析します。
//! 検索フォーム、検索結果一覧、詳細表示の3つの主要機能をサポートします。
//!
//! 各機能は三層アーキテクチャで構成されています：
//! - `Builder`: パーサーの設定値を管理し、パーサーインスタンスを生成
//! - `Parser`: HTMLの解析ロジックを実装
//! - `Model`: 抽出されたデータを表現するDTO/構造体

/// シラバス検索フォーム（検索条件の設定）
pub mod search;

/// シラバス検索結果（科目一覧とページネーション）
pub mod search_result;

/// シラバス詳細表示（個別科目の詳細情報）
pub mod view;

use crate::common::traits::PageParser;
use scraper::Html;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// WASM用のシラバスパーサー
///
/// シラバス関連のページを解析するための統合パーサーです。
/// 内部的に三層アーキテクチャ化されたパーサーを使用します。
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct SyllabusParser {
    search_result: search_result::SyllabusSearchResultParser,
    view: view::SyllabusViewParser,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl SyllabusParser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<SyllabusParser, JsError> {
        Ok(Self {
            search_result: search_result::SyllabusSearchResultParser::new()?,
            view: view::SyllabusViewParser::new()?,
        })
    }

    /// HTMLコンテンツからシラバス検索結果を解析
    pub fn parse_search_result(
        &self,
        html_content: &str,
    ) -> Result<search_result::SyllabusSearchResultPage, JsError> {
        let document = Html::parse_document(html_content);
        Ok(self.search_result.parse_document(&document)?)
    }

    /// HTMLコンテンツからシラバス詳細情報を解析
    pub fn parse_view(&self, html_content: &str) -> Result<view::LessonInfo, JsError> {
        let document = Html::parse_document(html_content);
        Ok(self.view.parse_document(&document)?)
    }
}
