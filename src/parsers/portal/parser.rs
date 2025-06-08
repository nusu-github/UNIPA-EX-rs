//! ポータル（メイン画面）のパーサー実装
//!
//! UNIVERSAL PASSPORT EXのメイン画面のHTML解析ロジックを提供します。
//! カレンダー、スケジュール、お気に入りリンク、お知らせなどの
//! 主要コンポーネントを解析します。

use scraper::{Html, Selector};

use crate::common::traits::PageParser;
use crate::utils::error::ParseError;
use super::model::{
    Portal, Calendar, Schedule, FavoriteLinks, Notifications,
    ScheduleEntry, FavoriteLink, NotificationSection, NotificationEntry,
};

/// ポータルパーサー
pub struct PortalParser;

impl PortalParser {
    /// 新しいパーサーインスタンスを作成
    pub fn new() -> Result<Self, ParseError> {
        Ok(Self)
    }

    /// カレンダー情報を解析する
    fn parse_calendar(&self, document: &Html) -> Result<Calendar, ParseError> {
        let mut calendar = Calendar::default();

        // カレンダーボタンを解析
        let button_selector = Selector::parse("input[type='image']").map_err(|e| {
            ParseError::selector_creation_failed("input[type='image']", &e.to_string())
        })?;

        for button in document.select(&button_selector) {
            if let Some(alt) = button.value().attr("alt") {
                match alt {
                    s if s.contains("前月") => {
                        calendar.last_month_button = alt.to_string();
                    }
                    s if s.contains("次月") => {
                        calendar.next_month_button = alt.to_string();
                    }
                    s if s.contains("今日") || s.contains("本日") => {
                        calendar.current_day_button = alt.to_string();
                    }
                    s if s.contains("月間") => {
                        calendar.month_schedule_button = alt.to_string();
                    }
                    _ => {}
                }
            }
        }

        // 年度・学期情報を解析
        let year_semester_selector = Selector::parse(".style24")
            .map_err(|e| ParseError::selector_creation_failed(".style24", &e.to_string()))?;

        if let Some(year_semester) = document.select(&year_semester_selector).next() {
            let text = year_semester.inner_html();
            // 例: "2025春学期" から年と学期を抽出
            if let Some(captures) = regex_lite::Regex::new(r"(\d{4})(.+)")
                .map_err(|e| {
                    ParseError::data_parsing_failed("year-semester regex", &e.to_string())
                })?
                .captures(&text)
            {
                calendar.year = captures[1].to_string();
                calendar.month = captures[2].to_string();
            }
        }

        // 隠しフィールドから現在日付と選択日付を取得
        let current_date_selector = Selector::parse("input[name='form1:Poa00101A:htmlCurDate']")
            .map_err(|e| ParseError::selector_creation_failed("input[name='form1:Poa00101A:htmlCurDate']", &e.to_string()))?;
        
        if let Some(current_date_input) = document.select(&current_date_selector).next() {
            calendar.current_date = current_date_input.value().attr("value").unwrap_or("").to_string();
        }

        let selected_day_selector = Selector::parse("input[name='form1:Poa00101A:htmlHidden_selectDay']")
            .map_err(|e| ParseError::selector_creation_failed("input[name='form1:Poa00101A:htmlHidden_selectDay']", &e.to_string()))?;
        
        if let Some(selected_day_input) = document.select(&selected_day_selector).next() {
            calendar.selected_day = selected_day_input.value().attr("value").unwrap_or("").to_string();
        }

        // カレンダーテーブルから日付グリッドを解析
        let calendar_table_selector = Selector::parse("#form1\\:Poa00101A\\:htmlCalendarTable")
            .map_err(|e| ParseError::selector_creation_failed("#form1:Poa00101A:htmlCalendarTable", &e.to_string()))?;

        if let Some(calendar_table) = document.select(&calendar_table_selector).next() {
            let row_selector = Selector::parse("tbody tr")
                .map_err(|e| ParseError::selector_creation_failed("tbody tr", &e.to_string()))?;

            for row in calendar_table.select(&row_selector) {
                let cell_selector = Selector::parse("td")
                    .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

                let mut week_days = Vec::new();
                
                for cell in row.select(&cell_selector) {
                    let mut day = super::model::CalendarDay::default();
                    
                    // セルIDを取得
                    if let Some(id) = cell.value().attr("id") {
                        day.cell_id = Some(id.to_string());
                    }

                    // CSSクラスを取得
                    if let Some(class) = cell.value().attr("class") {
                        day.css_classes = class.split_whitespace().map(|s| s.to_string()).collect();
                        day.is_today = day.css_classes.contains(&"todayColor".to_string());
                    }

                    // 日付数値を解析
                    let cell_text = cell.inner_html().trim().to_string();
                    if !cell_text.is_empty() && cell_text != "&nbsp;" {
                        if let Ok(day_num) = cell_text.parse::<u32>() {
                            day.day_number = Some(day_num);
                        }
                    }

                    // リンク先を取得（onclick属性から）
                    if let Some(onclick) = cell.value().attr("onclick") {
                        day.link = Some(onclick.to_string());
                    }

                    week_days.push(day);
                }
                
                if !week_days.is_empty() {
                    calendar.days.push(week_days);
                }
            }
        }

        Ok(calendar)
    }

    /// スケジュール情報を解析する
    fn parse_schedule(&self, document: &Html) -> Result<Schedule, ParseError> {
        let mut schedule = Schedule::default();
        schedule.title = "今日の時限割".to_string();

        // スケジュールテーブルを解析
        let schedule_table_selector = Selector::parse("#form1\\:Poa00401A\\:htmlTodayJikanTable")
            .map_err(|e| ParseError::selector_creation_failed("#form1:Poa00401A:htmlTodayJikanTable", &e.to_string()))?;

        if let Some(schedule_table) = document.select(&schedule_table_selector).next() {
            let row_selector = Selector::parse("tr")
                .map_err(|e| ParseError::selector_creation_failed("tr", &e.to_string()))?;

            for row in schedule_table.select(&row_selector) {
                let cell_selector = Selector::parse("td")
                    .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

                let _cells: Vec<_> = row.select(&cell_selector).collect();
                
                // 日付・時限情報（.date, .jigen）
                let date_jigen_selector = Selector::parse(".date, .jigen")
                    .map_err(|e| ParseError::selector_creation_failed(".date, .jigen", &e.to_string()))?;
                
                // 授業情報（.jugyo, .kyoin, .kyositu）
                let class_info_selector = Selector::parse(".jugyo, .kyoin, .kyositu")
                    .map_err(|e| ParseError::selector_creation_failed(".jugyo, .kyoin, .kyositu", &e.to_string()))?;

                let mut entry = ScheduleEntry::default();
                let mut has_content = false;

                // 日付・時限情報を取得
                for date_elem in row.select(&date_jigen_selector) {
                    let text = date_elem.inner_html().trim().to_string();
                    if !text.is_empty() {
                        if entry.date.is_empty() {
                            entry.date = text;
                        } else {
                            entry.date = format!("{} {}", entry.date, text);
                        }
                        has_content = true;
                    }
                }

                // 授業情報を取得
                let mut class_parts = Vec::new();
                for class_elem in row.select(&class_info_selector) {
                    let text = class_elem.inner_html().trim().to_string();
                    if !text.is_empty() {
                        class_parts.push(text);
                        has_content = true;
                    }
                }
                
                if !class_parts.is_empty() {
                    entry.class_content = class_parts.join(" ");
                }

                // 画像パス（区切り線など）を取得
                let img_selector = Selector::parse("img")
                    .map_err(|e| ParseError::selector_creation_failed("img", &e.to_string()))?;
                
                for img in row.select(&img_selector) {
                    if let Some(src) = img.value().attr("src") {
                        entry.image_path = Some(src.to_string());
                        break;
                    }
                }

                if has_content {
                    schedule.entries.push(entry);
                }
            }
        }

        // 隠しフィールドから選択授業コードを取得
        let hidden_selector = Selector::parse("input[type='hidden'][name*='htmlJugyoListState']")
            .map_err(|e| {
            ParseError::selector_creation_failed(
                "input[type='hidden'][name*='htmlJugyoListState']",
                &e.to_string(),
            )
        })?;

        if let Some(hidden) = document.select(&hidden_selector).next() {
            schedule.selected_class_code = hidden.value().attr("value").unwrap_or("").to_string();
        }

        Ok(schedule)
    }

    /// お気に入りリンクを解析する
    fn parse_favorite_links(&self, document: &Html) -> Result<FavoriteLinks, ParseError> {
        let mut favorite_links = FavoriteLinks::default();
        favorite_links.title = "お気に入りリンク".to_string();
        favorite_links.edit_button = "編集".to_string();

        // お気に入りリンクテーブルを解析
        let link_table_selector = Selector::parse("#form1\\:Poa00301A\\:htmlPrjTable")
            .map_err(|e| ParseError::selector_creation_failed("#form1:Poa00301A:htmlPrjTable", &e.to_string()))?;

        if let Some(link_table) = document.select(&link_table_selector).next() {
            let row_selector = Selector::parse("tr")
                .map_err(|e| ParseError::selector_creation_failed("tr", &e.to_string()))?;

            for (row_index, row) in link_table.select(&row_selector).enumerate() {
                let link_selector = Selector::parse("a")
                    .map_err(|e| ParseError::selector_creation_failed("a", &e.to_string()))?;

                for link in row.select(&link_selector) {
                    let mut link_item = FavoriteLink::default();
                    
                    // リンク名を取得
                    link_item.name = link.inner_html()
                        .replace("&nbsp;", " ")
                        .trim()
                        .to_string();

                    if !link_item.name.is_empty() {
                        // 隠しフィールドからURL、パラメータ、メソッドを取得
                        let url_selector = Selector::parse(&format!("input[name*='htmlLinkUrl'][name*='{}']", row_index))
                            .map_err(|e| ParseError::selector_creation_failed(&format!("input[name*='htmlLinkUrl'][name*='{}']", row_index), &e.to_string()))?;
                        
                        if let Some(url_input) = document.select(&url_selector).next() {
                            link_item.url = url_input.value().attr("value").unwrap_or("").to_string();
                        }

                        let params_selector = Selector::parse(&format!("input[name*='htmlLinkPrm'][name*='{}']", row_index))
                            .map_err(|e| ParseError::selector_creation_failed(&format!("input[name*='htmlLinkPrm'][name*='{}']", row_index), &e.to_string()))?;
                        
                        if let Some(params_input) = document.select(&params_selector).next() {
                            link_item.params = params_input.value().attr("value").unwrap_or("").to_string();
                        }

                        let method_selector = Selector::parse(&format!("input[name*='htmlLinkMtd'][name*='{}']", row_index))
                            .map_err(|e| ParseError::selector_creation_failed(&format!("input[name*='htmlLinkMtd'][name*='{}']", row_index), &e.to_string()))?;
                        
                        if let Some(method_input) = document.select(&method_selector).next() {
                            link_item.method = method_input.value().attr("value").unwrap_or("POST").to_string();
                        }

                        // onclickからJavaScript呼び出しを取得（フォールバック）
                        if link_item.url.is_empty() {
                            if let Some(onclick) = link.value().attr("onclick") {
                                link_item.url = onclick.to_string();
                                link_item.method = "POST".to_string();
                            }
                        }

                        favorite_links.links.push(link_item);
                    }
                }
            }
        }

        Ok(favorite_links)
    }

    /// お知らせ情報を解析する
    fn parse_notifications(&self, document: &Html) -> Result<Notifications, ParseError> {
        let mut notifications = Notifications::default();
        notifications.all_info_button = "全て表示".to_string();

        // お知らせ親テーブルを解析
        let parent_table_selector = Selector::parse("#form1\\:Poa00201A\\:htmlParentTable")
            .map_err(|e| ParseError::selector_creation_failed("#form1:Poa00201A:htmlParentTable", &e.to_string()))?;

        if let Some(_parent_table) = document.select(&parent_table_selector).next() {
            // セクション別テーブルを解析（お知らせ、遠隔授業、授業連絡、就職支援課）
            let section_configs = vec![
                ("0", "お知らせ"),
                ("1", "遠隔授業"),
                ("2", "授業連絡"),
                ("3", "就職支援課"),
            ];

            for (section_index, section_title) in section_configs {
                let section_table_selector = Selector::parse(&format!("#form1\\:Poa00201A\\:htmlParentTable\\:{}\\:htmlDetailTbl", section_index))
                    .map_err(|e| ParseError::selector_creation_failed(&format!("#form1:Poa00201A:htmlParentTable:{}:htmlDetailTbl", section_index), &e.to_string()))?;

                if let Some(section_table) = document.select(&section_table_selector).next() {
                    let mut section = NotificationSection::default();
                    section.header_title = section_title.to_string();
                    section.section_id = section_index.to_string();
                    section.display_mode = "summary".to_string();
                    section.has_all_button = true;

                    // テーブル行を解析
                    let row_selector = Selector::parse("tr")
                        .map_err(|e| ParseError::selector_creation_failed("tr", &e.to_string()))?;

                    for row in section_table.select(&row_selector) {
                        let cell_selector = Selector::parse("td")
                            .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

                        let _cells: Vec<_> = row.select(&cell_selector).collect();
                        
                        if _cells.len() >= 3 {
                            let mut entry = NotificationEntry::default();

                            // 既読/未読アイコン
                            let img_selector = Selector::parse("img")
                                .map_err(|e| ParseError::selector_creation_failed("img", &e.to_string()))?;
                            
                            if let Some(img) = _cells[0].select(&img_selector).next() {
                                if let Some(src) = img.value().attr("src") {
                                    if src.contains("read") || src.contains("未読") {
                                        entry.read_status_image = Some(src.to_string());
                                    } else if src.contains("important") || src.contains("重要") {
                                        entry.important_status_image = Some(src.to_string());
                                    }
                                }
                            }

                            // タイトル
                            let title_text = _cells[1].inner_html().trim().to_string();
                            if !title_text.is_empty() && title_text != "&nbsp;" {
                                entry.title = Some(title_text);
                            }

                            // 情報源
                            if _cells.len() > 2 {
                                let source_text = _cells[2].inner_html().trim().to_string();
                                if !source_text.is_empty() && source_text != "&nbsp;" {
                                    entry.source = Some(source_text);
                                }
                            }

                            // 掲載日
                            if _cells.len() > 3 {
                                entry.insert_date = _cells[3].inner_html().trim().to_string();
                            }

                            section.entries.push(entry);
                        }
                    }

                    section.total_count = section.entries.len().to_string();

                    // コメント情報を取得（セクションヘッダーから）
                    let comment_selector = Selector::parse(".comment, .note")
                        .map_err(|e| ParseError::selector_creation_failed(".comment, .note", &e.to_string()))?;
                    
                    if let Some(comment_elem) = section_table.select(&comment_selector).next() {
                        let comment_text = comment_elem.inner_html().trim().to_string();
                        if !comment_text.is_empty() {
                            section.comment = Some(comment_text);
                        }
                    }

                    notifications.sections.push(section);
                }
            }
        }

        Ok(notifications)
    }
}

impl PageParser<Portal> for PortalParser {
    const PAGE_TYPE: &'static str = "ポータル";

    fn parse_document(&self, document: &Html) -> Result<Portal, ParseError> {
        let calendar = self.parse_calendar(document)?;
        let schedule = self.parse_schedule(document)?;
        let favorite_links = self.parse_favorite_links(document)?;
        let notifications = self.parse_notifications(document)?;

        Ok(Portal {
            calendar,
            schedule,
            favorite_links,
            notifications,
        })
    }
}

/// お知らせ全表示パーサー
pub struct PortalAllNotificationsParser;

impl PortalAllNotificationsParser {
    /// 新しいパーサーインスタンスを作成
    pub fn new() -> Result<Self, ParseError> {
        Ok(Self)
    }

    /// お知らせ全表示情報を解析する
    fn parse_all_notifications(&self, document: &Html) -> Result<Notifications, ParseError> {
        let mut notifications = Notifications::default();
        notifications.all_info_button = "全て表示".to_string();

        // お知らせ親テーブルを解析
        let parent_table_selector = Selector::parse("#form1\\:Poa00201A\\:htmlParentTable")
            .map_err(|e| ParseError::selector_creation_failed("#form1:Poa00201A:htmlParentTable", &e.to_string()))?;

        if let Some(_parent_table) = document.select(&parent_table_selector).next() {
            // セクション別テーブルを解析（お知らせ、遠隔授業、授業連絡、就職支援課）
            let section_configs = vec![
                ("0", "お知らせ"),
                ("1", "遠隔授業"),
                ("2", "授業連絡"),
                ("3", "就職支援課"),
            ];

            for (section_index, section_title) in section_configs {
                let section_table_selector = Selector::parse(&format!("#form1\\:Poa00201A\\:htmlParentTable\\:{}\\:htmlDetailTbl", section_index))
                    .map_err(|e| ParseError::selector_creation_failed(&format!("#form1:Poa00201A:htmlParentTable:{}:htmlDetailTbl", section_index), &e.to_string()))?;

                if let Some(section_table) = document.select(&section_table_selector).next() {
                    let mut section = NotificationSection::default();
                    section.header_title = section_title.to_string();
                    section.section_id = section_index.to_string();
                    section.display_mode = "all".to_string(); // 全表示モード
                    section.has_all_button = false; // 全表示時は全てボタンを非表示

                    // テーブル行を解析（全件取得）
                    let row_selector = Selector::parse("tr")
                        .map_err(|e| ParseError::selector_creation_failed("tr", &e.to_string()))?;

                    for row in section_table.select(&row_selector) {
                        let cell_selector = Selector::parse("td")
                            .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

                        let _cells: Vec<_> = row.select(&cell_selector).collect();
                        
                        if _cells.len() >= 3 {
                            let mut entry = NotificationEntry::default();

                            // 既読/未読アイコン
                            let img_selector = Selector::parse("img")
                                .map_err(|e| ParseError::selector_creation_failed("img", &e.to_string()))?;
                            
                            if let Some(img) = _cells[0].select(&img_selector).next() {
                                if let Some(src) = img.value().attr("src") {
                                    if src.contains("read") || src.contains("未読") {
                                        entry.read_status_image = Some(src.to_string());
                                    } else if src.contains("important") || src.contains("重要") {
                                        entry.important_status_image = Some(src.to_string());
                                    }
                                }
                            }

                            // タイトル
                            let title_text = _cells[1].inner_html().trim().to_string();
                            if !title_text.is_empty() && title_text != "&nbsp;" {
                                entry.title = Some(title_text);
                            }

                            // 情報源
                            if _cells.len() > 2 {
                                let source_text = _cells[2].inner_html().trim().to_string();
                                if !source_text.is_empty() && source_text != "&nbsp;" {
                                    entry.source = Some(source_text);
                                }
                            }

                            // 掲載日
                            if _cells.len() > 3 {
                                entry.insert_date = _cells[3].inner_html().trim().to_string();
                            }

                            section.entries.push(entry);
                        }
                    }

                    section.total_count = section.entries.len().to_string();

                    // コメント情報を取得
                    let comment_selector = Selector::parse(".comment, .note")
                        .map_err(|e| ParseError::selector_creation_failed(".comment, .note", &e.to_string()))?;
                    
                    if let Some(comment_elem) = section_table.select(&comment_selector).next() {
                        let comment_text = comment_elem.inner_html().trim().to_string();
                        if !comment_text.is_empty() {
                            section.comment = Some(comment_text);
                        }
                    }

                    notifications.sections.push(section);
                }
            }
        }

        Ok(notifications)
    }
}

impl PageParser<Portal> for PortalAllNotificationsParser {
    const PAGE_TYPE: &'static str = "ポータル（お知らせ全表示）";

    fn parse_document(&self, document: &Html) -> Result<Portal, ParseError> {
        let base_parser = PortalParser::new()?;
        let calendar = base_parser.parse_calendar(document)?;
        let schedule = base_parser.parse_schedule(document)?;
        let favorite_links = base_parser.parse_favorite_links(document)?;
        let notifications = self.parse_all_notifications(document)?;

        Ok(Portal {
            calendar,
            schedule,
            favorite_links,
            notifications,
        })
    }
}

/// 授業連絡表示パーサー
pub struct PortalClassContactParser;

impl PortalClassContactParser {
    /// 新しいパーサーインスタンスを作成
    pub fn new() -> Result<Self, ParseError> {
        Ok(Self)
    }

    /// 授業連絡情報を解析する
    fn parse_class_contact(&self, document: &Html) -> Result<Notifications, ParseError> {
        let mut notifications = Notifications::default();
        notifications.all_info_button = "全て表示".to_string();

        // お知らせ親テーブルを解析
        let parent_table_selector = Selector::parse("#form1\\:Poa00201A\\:htmlParentTable")
            .map_err(|e| ParseError::selector_creation_failed("#form1:Poa00201A:htmlParentTable", &e.to_string()))?;

        if let Some(_parent_table) = document.select(&parent_table_selector).next() {
            // 授業連絡セクションのみを解析（セクション2）
            let section_table_selector = Selector::parse("#form1\\:Poa00201A\\:htmlParentTable\\:2\\:htmlDetailTbl")
                .map_err(|e| ParseError::selector_creation_failed("#form1:Poa00201A:htmlParentTable:2:htmlDetailTbl", &e.to_string()))?;

            if let Some(section_table) = document.select(&section_table_selector).next() {
                let mut section = NotificationSection::default();
                section.header_title = "授業連絡".to_string();
                section.section_id = "2".to_string();
                section.display_mode = "summary".to_string();
                section.has_all_button = true;

                // テーブル行を解析
                let row_selector = Selector::parse("tr")
                    .map_err(|e| ParseError::selector_creation_failed("tr", &e.to_string()))?;

                for row in section_table.select(&row_selector) {
                    let cell_selector = Selector::parse("td")
                        .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

                    let _cells: Vec<_> = row.select(&cell_selector).collect();
                    
                    if _cells.len() >= 3 {
                        let mut entry = NotificationEntry::default();

                        // 既読/未読アイコン
                        let img_selector = Selector::parse("img")
                            .map_err(|e| ParseError::selector_creation_failed("img", &e.to_string()))?;
                        
                        if let Some(img) = _cells[0].select(&img_selector).next() {
                            if let Some(src) = img.value().attr("src") {
                                if src.contains("read") || src.contains("未読") {
                                    entry.read_status_image = Some(src.to_string());
                                } else if src.contains("important") || src.contains("重要") {
                                    entry.important_status_image = Some(src.to_string());
                                }
                            }
                        }

                        // タイトル
                        let title_text = _cells[1].inner_html().trim().to_string();
                        if !title_text.is_empty() && title_text != "&nbsp;" {
                            entry.title = Some(title_text);
                        }

                        // 情報源
                        if _cells.len() > 2 {
                            let source_text = _cells[2].inner_html().trim().to_string();
                            if !source_text.is_empty() && source_text != "&nbsp;" {
                                entry.source = Some(source_text);
                            }
                        }

                        // 掲載日
                        if _cells.len() > 3 {
                            entry.insert_date = _cells[3].inner_html().trim().to_string();
                        }

                        section.entries.push(entry);
                    }
                }

                section.total_count = section.entries.len().to_string();

                // コメント情報を取得
                let comment_selector = Selector::parse(".comment, .note")
                    .map_err(|e| ParseError::selector_creation_failed(".comment, .note", &e.to_string()))?;
                
                if let Some(comment_elem) = section_table.select(&comment_selector).next() {
                    let comment_text = comment_elem.inner_html().trim().to_string();
                    if !comment_text.is_empty() {
                        section.comment = Some(comment_text);
                    }
                }

                notifications.sections.push(section);
            }
        }

        Ok(notifications)
    }
}

impl PageParser<Portal> for PortalClassContactParser {
    const PAGE_TYPE: &'static str = "ポータル（授業連絡表示）";

    fn parse_document(&self, document: &Html) -> Result<Portal, ParseError> {
        let base_parser = PortalParser::new()?;
        let calendar = base_parser.parse_calendar(document)?;
        let schedule = base_parser.parse_schedule(document)?;
        let favorite_links = base_parser.parse_favorite_links(document)?;
        let notifications = self.parse_class_contact(document)?;

        Ok(Portal {
            calendar,
            schedule,
            favorite_links,
            notifications,
        })
    }
}

/// 授業連絡全表示パーサー
pub struct PortalAllClassContactParser;

impl PortalAllClassContactParser {
    /// 新しいパーサーインスタンスを作成
    pub fn new() -> Result<Self, ParseError> {
        Ok(Self)
    }

    /// 授業連絡全表示情報を解析する
    fn parse_all_class_contact(&self, document: &Html) -> Result<Notifications, ParseError> {
        let mut notifications = Notifications::default();
        notifications.all_info_button = "全て表示".to_string();

        // お知らせ親テーブルを解析
        let parent_table_selector = Selector::parse("#form1\\:Poa00201A\\:htmlParentTable")
            .map_err(|e| ParseError::selector_creation_failed("#form1:Poa00201A:htmlParentTable", &e.to_string()))?;

        if let Some(_parent_table) = document.select(&parent_table_selector).next() {
            // 授業連絡セクションのみを解析（セクション2）
            let section_table_selector = Selector::parse("#form1\\:Poa00201A\\:htmlParentTable\\:2\\:htmlDetailTbl")
                .map_err(|e| ParseError::selector_creation_failed("#form1:Poa00201A:htmlParentTable:2:htmlDetailTbl", &e.to_string()))?;

            if let Some(section_table) = document.select(&section_table_selector).next() {
                let mut section = NotificationSection::default();
                section.header_title = "授業連絡".to_string();
                section.section_id = "2".to_string();
                section.display_mode = "all".to_string(); // 全表示モード
                section.has_all_button = false; // 全表示時は全てボタンを非表示

                // テーブル行を解析（全件取得）
                let row_selector = Selector::parse("tr")
                    .map_err(|e| ParseError::selector_creation_failed("tr", &e.to_string()))?;

                for row in section_table.select(&row_selector) {
                    let cell_selector = Selector::parse("td")
                        .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

                    let _cells: Vec<_> = row.select(&cell_selector).collect();
                    
                    if _cells.len() >= 3 {
                        let mut entry = NotificationEntry::default();

                        // 既読/未読アイコン
                        let img_selector = Selector::parse("img")
                            .map_err(|e| ParseError::selector_creation_failed("img", &e.to_string()))?;
                        
                        if let Some(img) = _cells[0].select(&img_selector).next() {
                            if let Some(src) = img.value().attr("src") {
                                if src.contains("read") || src.contains("未読") {
                                    entry.read_status_image = Some(src.to_string());
                                } else if src.contains("important") || src.contains("重要") {
                                    entry.important_status_image = Some(src.to_string());
                                }
                            }
                        }

                        // タイトル
                        let title_text = _cells[1].inner_html().trim().to_string();
                        if !title_text.is_empty() && title_text != "&nbsp;" {
                            entry.title = Some(title_text);
                        }

                        // 情報源
                        if _cells.len() > 2 {
                            let source_text = _cells[2].inner_html().trim().to_string();
                            if !source_text.is_empty() && source_text != "&nbsp;" {
                                entry.source = Some(source_text);
                            }
                        }

                        // 掲載日
                        if _cells.len() > 3 {
                            entry.insert_date = _cells[3].inner_html().trim().to_string();
                        }

                        section.entries.push(entry);
                    }
                }

                section.total_count = section.entries.len().to_string();

                // コメント情報を取得
                let comment_selector = Selector::parse(".comment, .note")
                    .map_err(|e| ParseError::selector_creation_failed(".comment, .note", &e.to_string()))?;
                
                if let Some(comment_elem) = section_table.select(&comment_selector).next() {
                    let comment_text = comment_elem.inner_html().trim().to_string();
                    if !comment_text.is_empty() {
                        section.comment = Some(comment_text);
                    }
                }

                notifications.sections.push(section);
            }
        }

        Ok(notifications)
    }
}

impl PageParser<Portal> for PortalAllClassContactParser {
    const PAGE_TYPE: &'static str = "ポータル（授業連絡全表示）";

    fn parse_document(&self, document: &Html) -> Result<Portal, ParseError> {
        let base_parser = PortalParser::new()?;
        let calendar = base_parser.parse_calendar(document)?;
        let schedule = base_parser.parse_schedule(document)?;
        let favorite_links = base_parser.parse_favorite_links(document)?;
        let notifications = self.parse_all_class_contact(document)?;

        Ok(Portal {
            calendar,
            schedule,
            favorite_links,
            notifications,
        })
    }
} 