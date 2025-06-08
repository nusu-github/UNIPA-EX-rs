//! ポータル（メイン画面）関連のページパーサー
//!
//! UNIVERSAL PASSPORT EXのメイン画面に表示される各種情報を解析します。
//! カレンダー、スケジュール、お気に入りリンク、お知らせなどの
//! 主要コンポーネントを含みます。

pub mod builder;
pub mod parser;
pub mod model;
pub mod notification_detail;

use crate::common::traits::PageParser;
use scraper::Html;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct PortalParser {
    parser: parser::PortalParser,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl PortalParser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Self, JsError> {
        Ok(Self {
            parser: builder::PortalParserBuilder::new().build()?,
        })
    }

    pub fn portal(&self, html_content: &str) -> Result<model::Portal, JsError> {
        Ok(self
            .parser
            .parse_document(&Html::parse_document(html_content))?)
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct PortalAllNotificationsParser {
    parser: parser::PortalAllNotificationsParser,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl PortalAllNotificationsParser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Self, JsError> {
        Ok(Self {
            parser: builder::PortalParserBuilder::new().build_all_notifications()?,
        })
    }

    pub fn portal_all_notifications(&self, html_content: &str) -> Result<model::Portal, JsError> {
        Ok(self
            .parser
            .parse_document(&Html::parse_document(html_content))?)
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct PortalClassContactParser {
    parser: parser::PortalClassContactParser,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl PortalClassContactParser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Self, JsError> {
        Ok(Self {
            parser: builder::PortalParserBuilder::new().build_class_contact()?,
        })
    }

    pub fn portal_class_contact(&self, html_content: &str) -> Result<model::Portal, JsError> {
        Ok(self
            .parser
            .parse_document(&Html::parse_document(html_content))?)
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct PortalAllClassContactParser {
    parser: parser::PortalAllClassContactParser,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl PortalAllClassContactParser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Self, JsError> {
        Ok(Self {
            parser: builder::PortalParserBuilder::new().build_all_class_contact()?,
        })
    }

    pub fn portal_all_class_contact(&self, html_content: &str) -> Result<model::Portal, JsError> {
        Ok(self
            .parser
            .parse_document(&Html::parse_document(html_content))?)
    }
}
