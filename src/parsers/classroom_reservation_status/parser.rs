//! 教室予約状況パーサー実装
//!
//! HTMLドキュメントから教室予約状況情報を抽出するパースロジックを提供します。

use scraper::{Html, Selector};

use super::model::{
    ClassroomReservationCell, ClassroomReservationStatus, ClassroomReservationTableData,
    ConflictingClass, PopupClassDetail, PopupClassroomDetail, PopupDuplicateClassDetail,
    PopupReservationDetail, ReservationType, SearchParams,
};
use crate::utils::error::ParseError;

/// 教室予約状況パーサー実装
pub struct ClassroomReservationStatusParserImpl {
    /// デバッグモード
    debug_mode: bool,
    /// 厳密なパースモード
    strict_mode: bool,
}

impl ClassroomReservationStatusParserImpl {
    /// 新しいパーサーインスタンスを作成
    pub fn new_with_config(debug_mode: bool, strict_mode: bool) -> Self {
        Self {
            debug_mode,
            strict_mode,
        }
    }

    /// HTMLドキュメントから教室予約状況を解析する
    pub fn parse_document(
        &self,
        document: &Html,
    ) -> Result<ClassroomReservationStatus, ParseError> {
        let search_params = self.parse_search_params(document)?;
        let reservation_table_data = self.parse_reservation_table_data(document)?;
        let popup_reservation_detail = self.parse_popup_reservation_detail(document)?;
        let popup_class_detail = self.parse_popup_class_detail(document)?;
        let popup_duplicate_class_detail = self.parse_popup_duplicate_class_detail(document)?;
        let popup_classroom_detail = self.parse_popup_classroom_detail(document)?;

        Ok(ClassroomReservationStatus {
            search_params,
            reservation_table_data,
            popup_reservation_detail,
            popup_class_detail,
            popup_duplicate_class_detail,
            popup_classroom_detail,
        })
    }

    /// ページ上部の検索条件を抽出する
    fn parse_search_params(&self, document: &Html) -> Result<SearchParams, ParseError> {
        let form_selector =
            Selector::parse("form, .search-form, .condition-form").map_err(|e| {
                ParseError::selector_creation_failed("form search selector", &e.to_string())
            })?;

        let input_selector = Selector::parse("input")
            .map_err(|e| ParseError::selector_creation_failed("input", &e.to_string()))?;

        let select_selector = Selector::parse("select")
            .map_err(|e| ParseError::selector_creation_failed("select", &e.to_string()))?;

        let option_selector = Selector::parse("option")
            .map_err(|e| ParseError::selector_creation_failed("option", &e.to_string()))?;

        let mut search_params = SearchParams::default();

        // フォーム要素を探す
        for form_element in document.select(&form_selector) {
            // input要素の処理
            for input in form_element.select(&input_selector) {
                if let Some(name) = input.value().attr("name") {
                    let value = input.value().attr("value").unwrap_or("").to_string();

                    match name {
                        n if n.contains("year") || n.contains("nendo") => {
                            search_params.academic_year = value;
                        }
                        n if n.contains("semester") || n.contains("gakki") => {
                            search_params.semester = value;
                        }
                        n if n.contains("dayofweek") || n.contains("youbi") => {
                            search_params.day_of_week = value;
                        }
                        n if n.contains("period") || n.contains("jigen") => {
                            search_params.period = value;
                        }
                        n if n.contains("subject") || n.contains("kamoku") => {
                            search_params.subject_name = value;
                        }
                        n if n.contains("instructor") || n.contains("kyoin") => {
                            search_params.instructor_name = value;
                        }
                        _ => {}
                    }
                }
            }

            // select要素の処理
            for select in form_element.select(&select_selector) {
                if let Some(name) = select.value().attr("name") {
                    // 選択されているoptionを探す
                    let selected_value = select
                        .select(&option_selector)
                        .find(|option| option.value().attr("selected").is_some())
                        .and_then(|option| option.value().attr("value"))
                        .unwrap_or("")
                        .to_string();

                    match name {
                        n if n.contains("building") || n.contains("tou") => {
                            search_params.building = selected_value;
                        }
                        n if n.contains("classroom") || n.contains("kyoshitsu") => {
                            search_params.classroom = selected_value;
                        }
                        n if n.contains("year") || n.contains("nendo") => {
                            search_params.academic_year = selected_value;
                        }
                        n if n.contains("semester") || n.contains("gakki") => {
                            search_params.semester = selected_value;
                        }
                        n if n.contains("dayofweek") || n.contains("youbi") => {
                            search_params.day_of_week = selected_value;
                        }
                        n if n.contains("period") || n.contains("jigen") => {
                            search_params.period = selected_value;
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(search_params)
    }

    /// 教室予約状況表示テーブルのデータを抽出する
    fn parse_reservation_table_data(
        &self,
        document: &Html,
    ) -> Result<Vec<ClassroomReservationTableData>, ParseError> {
        let table_selector = Selector::parse("table.reservationTable, table.timeTable, table")
            .map_err(|e| ParseError::selector_creation_failed("table", &e.to_string()))?;

        let row_selector = Selector::parse("tbody tr, tr")
            .map_err(|e| ParseError::selector_creation_failed("tr", &e.to_string()))?;

        let cell_selector = Selector::parse("td")
            .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

        let mut table_data = Vec::new();

        if let Some(table) = document.select(&table_selector).next() {
            for row in table.select(&row_selector) {
                let cells: Vec<_> = row.select(&cell_selector).collect();

                // 時限+7曜日のセルが存在する場合
                if cells.len() >= 8 {
                    let period = cells[0].inner_html().trim().to_string();

                    let reservation_row = ClassroomReservationTableData {
                        period,
                        monday: self.parse_reservation_cell(&cells[1])?,
                        tuesday: self.parse_reservation_cell(&cells[2])?,
                        wednesday: self.parse_reservation_cell(&cells[3])?,
                        thursday: self.parse_reservation_cell(&cells[4])?,
                        friday: self.parse_reservation_cell(&cells[5])?,
                        saturday: self.parse_reservation_cell(&cells[6])?,
                        sunday: self.parse_reservation_cell(&cells[7])?,
                    };

                    table_data.push(reservation_row);
                }
            }
        }

        Ok(table_data)
    }

    /// 個別の予約セルを解析する
    fn parse_reservation_cell(
        &self,
        cell: &scraper::ElementRef,
    ) -> Result<ClassroomReservationCell, ParseError> {
        let _cell_html = cell.inner_html();
        let cell_text = self.extract_cell_text_with_breaks(cell);

        // セルのスタイル情報を取得
        let cell_style = cell
            .value()
            .attr("class")
            .or_else(|| cell.value().attr("style"))
            .map(|s| s.to_string());

        // リンク要素があるかチェック
        let link_selector = Selector::parse("a")
            .map_err(|e| ParseError::selector_creation_failed("a", &e.to_string()))?;

        let detail_link_url = cell
            .select(&link_selector)
            .next()
            .and_then(|link| link.value().attr("href"))
            .map(|href| href.to_string());

        // 予約タイプを判定
        let reservation_type = self.determine_reservation_type(&cell_text, &cell_style)?;

        // 科目名と担当教員名を抽出
        let (subject_name, instructor_name, classroom_name) =
            self.extract_cell_content(&cell_text)?;

        Ok(ClassroomReservationCell {
            reservation_type,
            subject_name,
            instructor_name,
            classroom_name,
            detail_link_url,
            cell_style,
        })
    }

    /// セルの内容から予約タイプを判定する
    fn determine_reservation_type(
        &self,
        cell_text: &str,
        cell_style: &Option<String>,
    ) -> Result<ReservationType, ParseError> {
        // 空のセルは利用可能
        if cell_text.trim().is_empty() {
            return Ok(ReservationType::Available);
        }

        // スタイルによる判定
        if let Some(style) = cell_style {
            if style.contains("conflict") || style.contains("duplicate") {
                return Ok(ReservationType::Duplicate);
            }
            if style.contains("unavailable") || style.contains("disabled") {
                return Ok(ReservationType::Unavailable);
            }
            if style.contains("intensive") {
                return Ok(ReservationType::IntensiveCourse);
            }
        }

        // テキスト内容による判定
        if cell_text.contains("集中") || cell_text.contains("intensive") {
            return Ok(ReservationType::IntensiveCourse);
        }
        if cell_text.contains("重複") || cell_text.contains("conflict") {
            return Ok(ReservationType::Duplicate);
        }
        if cell_text.contains("利用不可") || cell_text.contains("unavailable") {
            return Ok(ReservationType::Unavailable);
        }

        // デフォルトは通常授業
        if !cell_text.trim().is_empty() {
            Ok(ReservationType::RegularClass)
        } else {
            Ok(ReservationType::Available)
        }
    }

    /// セルの内容から科目名、担当教員名、教室名を抽出する
    fn extract_cell_content(
        &self,
        cell_text: &str,
    ) -> Result<(Option<String>, Option<String>, Option<String>), ParseError> {
        if cell_text.trim().is_empty() {
            return Ok((None, None, None));
        }

        let lines: Vec<&str> = cell_text.lines().collect();

        let subject_name = lines
            .get(0)
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
        let instructor_name = lines
            .get(1)
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
        let classroom_name = lines
            .get(2)
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        Ok((subject_name, instructor_name, classroom_name))
    }

    /// セルのHTML内容を解析してテキスト内容を抽出する
    fn extract_cell_text_with_breaks(&self, cell: &scraper::ElementRef) -> String {
        // セルのHTMLを取得し、<br>タグを改行に変換する
        let html = cell.inner_html();

        // <br>タグを改行に置換
        let text_with_breaks = html
            .replace("<br>", "\n")
            .replace("<br/>", "\n")
            .replace("<BR>", "\n")
            .replace("<BR/>", "\n");

        // HTMLタグを除去してプレーンテキストを取得
        let temp_doc = scraper::Html::parse_fragment(&text_with_breaks);
        temp_doc.root_element().text().collect::<String>()
    }

    /// 予約詳細ポップアップから情報を抽出する
    fn parse_popup_reservation_detail(
        &self,
        document: &Html,
    ) -> Result<Option<PopupReservationDetail>, ParseError> {
        let popup_selector =
            Selector::parse(".popup-reservation, .reservation-detail, #reservationDetail")
                .map_err(|e| {
                    ParseError::selector_creation_failed("popup-reservation", &e.to_string())
                })?;

        if let Some(popup) = document.select(&popup_selector).next() {
            let mut detail = PopupReservationDetail::default();

            // ポップアップ内のテキストを解析
            let text_content = popup.text().collect::<String>();

            // 簡易的な解析（実際のHTMLに合わせて調整が必要）
            for line in text_content.lines() {
                let line = line.trim();
                if line.contains("予約ID") || line.contains("ID") {
                    detail.reservation_id = self.extract_value_after_colon(line);
                } else if line.contains("科目名") {
                    detail.subject_name = self.extract_value_after_colon(line);
                } else if line.contains("担当教員") {
                    detail.instructor_name = self.extract_value_after_colon(line);
                } else if line.contains("教室") {
                    detail.classroom_name = self.extract_value_after_colon(line);
                }
            }

            return Ok(Some(detail));
        }

        Ok(None)
    }

    /// 授業詳細ポップアップから情報を抽出する
    fn parse_popup_class_detail(
        &self,
        document: &Html,
    ) -> Result<Option<PopupClassDetail>, ParseError> {
        let popup_selector = Selector::parse(".popup-class, .class-detail, #classDetail")
            .map_err(|e| ParseError::selector_creation_failed("popup-class", &e.to_string()))?;

        if let Some(_popup) = document.select(&popup_selector).next() {
            // 実装省略 - 実際のHTMLに合わせて実装
            return Ok(None);
        }

        Ok(None)
    }

    /// 重複授業詳細ポップアップから情報を抽出する
    fn parse_popup_duplicate_class_detail(
        &self,
        document: &Html,
    ) -> Result<Option<PopupDuplicateClassDetail>, ParseError> {
        let popup_selector = Selector::parse(
            ".popup-duplicate, .duplicate-detail, #duplicateDetail",
        )
        .map_err(|e| ParseError::selector_creation_failed("popup-duplicate", &e.to_string()))?;

        if let Some(_popup) = document.select(&popup_selector).next() {
            // 実装省略 - 実際のHTMLに合わせて実装
            return Ok(None);
        }

        Ok(None)
    }

    /// 教室詳細ポップアップから情報を抽出する
    fn parse_popup_classroom_detail(
        &self,
        document: &Html,
    ) -> Result<Option<PopupClassroomDetail>, ParseError> {
        let popup_selector = Selector::parse(
            ".popup-classroom, .classroom-detail, #classroomDetail",
        )
        .map_err(|e| ParseError::selector_creation_failed("popup-classroom", &e.to_string()))?;

        if let Some(_popup) = document.select(&popup_selector).next() {
            // 実装省略 - 実際のHTMLに合わせて実装
            return Ok(None);
        }

        Ok(None)
    }

    /// コロンの後の値を抽出するヘルパー関数
    fn extract_value_after_colon(&self, text: &str) -> String {
        if let Some(pos) = text.find(':') {
            text[pos + 1..].trim().to_string()
        } else if let Some(pos) = text.find('：') {
            text[pos + 3..].trim().to_string() // 全角コロンは3バイト
        } else {
            text.trim().to_string()
        }
    }

    /// 日付文字列をISO 8601形式に変換する
    fn format_datetime(&self, datetime_str: &str) -> Result<String, ParseError> {
        if datetime_str.is_empty() {
            return Ok(String::new());
        }

        // 日本語の日付形式を解析してISO 8601形式に変換
        // 例: 「2025年1月15日 14:30」→「2025-01-15T14:30:00+09:00」
        let datetime_regex =
            regex_lite::Regex::new(r"(\d{4})年(\d{1,2})月(\d{1,2})日\s*(\d{1,2}):(\d{2})")
                .map_err(|e| ParseError::data_parsing_failed("datetime regex", &e.to_string()))?;

        if let Some(captures) = datetime_regex.captures(datetime_str) {
            let year = &captures[1];
            let month = format!("{:02}", captures[2].parse::<u32>().unwrap_or(1));
            let day = format!("{:02}", captures[3].parse::<u32>().unwrap_or(1));
            let hour = format!("{:02}", captures[4].parse::<u32>().unwrap_or(0));
            let minute = format!("{:02}", captures[5].parse::<u32>().unwrap_or(0));

            Ok(format!(
                "{}-{}-{}T{}:{}:00+09:00",
                year, month, day, hour, minute
            ))
        } else {
            Ok(datetime_str.to_string())
        }
    }
}
