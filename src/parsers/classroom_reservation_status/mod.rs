//! 教室予約状況関連のページパーサー
//!
//! UNIVERSAL PASSPORT EXの教室予約状況ページ（Ksc00101A）から情報を解析します。
//! 検索条件、予約状況テーブル、各種ポップアップ詳細などの機能をサポートします。

/// 教室予約状況のデータモデル
pub mod model;

/// 教室予約状況パーサー実装
pub mod parser;

/// 教室予約状況パーサービルダー
pub mod builder;

use crate::common::traits::PageParser;
use scraper::Html;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct ClassroomReservationStatusParser {
    parser: builder::ClassroomReservationStatusParserBuilder,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl ClassroomReservationStatusParser {
    /// 新しいパーサーインスタンスを作成
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            parser: builder::ClassroomReservationStatusParserBuilder::new(),
        }
    }

    /// デバッグモードを有効にしたパーサーインスタンスを作成
    #[wasm_bindgen]
    pub fn with_debug() -> Self {
        Self {
            parser: builder::ClassroomReservationStatusParserBuilder::new().with_debug_mode(true),
        }
    }

    /// 厳密なパースモードを有効にしたパーサーインスタンスを作成
    #[wasm_bindgen]
    pub fn with_strict_mode() -> Self {
        Self {
            parser: builder::ClassroomReservationStatusParserBuilder::new().with_strict_mode(true),
        }
    }

    /// HTMLコンテンツから教室予約状況を解析する
    #[wasm_bindgen]
    pub fn parse(&self, html_content: &str) -> Result<model::ClassroomReservationStatus, JsError> {
        Ok(self
            .parser
            .parse_document(&Html::parse_document(html_content))?)
    }
}
