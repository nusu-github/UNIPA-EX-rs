//! テスト解答状況関連のページパーサー
//!
//! UNIVERSAL PASSPORT EXのテスト解答一覧ページ（Stb00101A）から情報を抽出するための
//! 専用パーサーを提供します。未実施、実施中、実施済の各状態に対応しています。

/// テスト解答状況のデータ構造とパーサー実装
pub mod parser;

/// テスト解答状況のデータモデル
pub mod model;

/// テスト解答状況パーサーのビルダー
pub mod builder;

use crate::common::traits::PageParser;
use scraper::Html;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct TestAnswerStatusParser {
    parser: parser::TestAnswerStatusParserImpl,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl TestAnswerStatusParser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Self, JsError> {
        Ok(Self {
            parser: parser::TestAnswerStatusParserImpl::new()?,
        })
    }

    /// テスト解答状況をパースする
    pub fn parse(&self, html_content: &str) -> Result<model::TestAnswerStatus, JsError> {
        Ok(self
            .parser
            .parse_document(&Html::parse_document(html_content))?)
    }
}
