//! お知らせ詳細関連のページパーサー
//!
//! UNIVERSAL PASSPORT EXのお知らせ詳細ポップアップ画面の情報を解析します。
//! お知らせのタイトル、送信者、本文、添付ファイルなどの
//! 詳細情報を含みます。

pub mod builder;
pub mod parser;
pub mod model;

use crate::common::traits::PageParser;
use scraper::Html;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct NotificationDetailParser {
    parser: parser::NotificationDetailParser,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl NotificationDetailParser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Self, JsError> {
        Ok(Self {
            parser: builder::NotificationDetailParserBuilder::new().build()?,
        })
    }

    pub fn notification_detail(&self, html_content: &str) -> Result<model::NotificationDetail, JsError> {
        Ok(self
            .parser
            .parse_document(&Html::parse_document(html_content))?)
    }
}