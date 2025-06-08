pub mod calendar_view;
pub mod list_view;

use crate::common::traits::PageParser;
use scraper::Html;
use serde::{Deserialize, Serialize};
use tsify::Tsify;


#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct StudentTimetable {}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct StudentTimetableParser {
    calendar_view: calendar_view::CalendarViewParser,
    list_view: list_view::ListViewParser,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl StudentTimetableParser {
    pub fn new() -> Result<Self, JsError> {
        Ok(Self {
            calendar_view: calendar_view::CalendarViewParser::new()?,
            list_view: list_view::ListViewParser::new()?,
        })
    }

    pub fn calendar_view(&self, html_content: &str) -> Result<StudentTimetable, JsError> {
        Ok(self
            .calendar_view
            .parse_document(&Html::parse_document(html_content))?)
    }

    pub fn list_view(&self, html_content: &str) -> Result<StudentTimetable, JsError> {
        Ok(self
            .list_view
            .parse_document(&Html::parse_document(html_content))?)
    }
}
