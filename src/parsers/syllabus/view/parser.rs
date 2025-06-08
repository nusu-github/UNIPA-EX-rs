//! シラバス詳細表示のパーサー実装
//!
//! UNIVERSAL PASSPORT EXのシラバス詳細表示ページから授業情報を抽出するパーサーロジックを提供します。

use super::model::*;
use crate::common::traits::PageParser;
use crate::utils::error::ParseError;
use scraper::{Html, Selector};

/// シラバス詳細表示パーサー
pub struct SyllabusViewParser;

impl SyllabusViewParser {
    /// 新しいパーサーインスタンスを作成
    pub fn new() -> Result<Self, ParseError> {
        Ok(Self)
    }

    /// テーブル行から値を抽出するヘルパー関数
    fn extract_table_value(
        &self,
        document: &Html,
        header_text: &str,
    ) -> Result<String, ParseError> {
        let tr_selector = Selector::parse("tr")
            .map_err(|e| ParseError::selector_creation_failed("tr", &e.to_string()))?;
        let th_selector = Selector::parse("th")
            .map_err(|e| ParseError::selector_creation_failed("th", &e.to_string()))?;
        let td_selector = Selector::parse("td")
            .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

        // 各行を調べて、指定されたヘッダーテキストを含むTHがある行を探す
        for tr_element in document.select(&tr_selector) {
            let th_elements: Vec<_> = tr_element.select(&th_selector).collect();
            let td_elements: Vec<_> = tr_element.select(&td_selector).collect();

            // 小さなテーブル行のみを対象とする（大きな行は無視）
            if th_elements.len() > 10 || td_elements.len() > 10 {
                continue;
            }

            // この行で指定されたヘッダーテキストを含むTHを探し、対応するTDを見つける
            for (th_index, th_element) in th_elements.iter().enumerate() {
                let th_text = th_element.text().collect::<String>().trim().to_string();
                if th_text == header_text {
                    // 対応するTDを探す（THの位置を考慮してTDをマッピング）
                    // 同じ行の中でこのTHの後にあるTDを探す
                    if let Some(td_text) =
                        self.find_corresponding_td(&th_elements, &td_elements, th_index)
                    {
                        if !td_text.is_empty() && td_text != "&nbsp;" && td_text != "\u{a0}" {
                            return Ok(td_text);
                        }
                    }
                    break;
                }
            }
        }

        Err(ParseError::element_not_found(
            &format!("table value for '{}'", header_text),
            "syllabus table",
        ))
    }

    /// THのインデックスに対応するTDを見つける  
    fn find_corresponding_td(
        &self,
        _th_elements: &[scraper::ElementRef],
        td_elements: &[scraper::ElementRef],
        th_index: usize,
    ) -> Option<String> {
        // シンプルなアプローチ：同じ行では、THとTDが交互に配置されている場合が多い
        // HTMLでは "年度学期" TH → "2025年度 春学期" TD → "曜日時限" TH → "火曜3限 火曜4限" TD の順序

        // TH数に応じて適切なTDを選択
        match th_index {
            0 => {
                // 最初のTH → 最初のTD
                if !td_elements.is_empty() {
                    Some(td_elements[0].text().collect::<String>().trim().to_string())
                } else {
                    None
                }
            }
            1 => {
                // 2番目のTH → 2番目のTD（存在する場合）
                if td_elements.len() > 1 {
                    Some(td_elements[1].text().collect::<String>().trim().to_string())
                } else {
                    None
                }
            }
            _ => {
                // その他の場合はTHインデックスに対応するTDを探す（フォールバック）
                if th_index < td_elements.len() {
                    Some(
                        td_elements[th_index]
                            .text()
                            .collect::<String>()
                            .trim()
                            .to_string(),
                    )
                } else {
                    None
                }
            }
        }
    }

    /// オプション値を抽出（空文字やnbspの場合はNoneを返す）
    fn extract_optional_value(&self, document: &Html, header_text: &str) -> Option<String> {
        match self.extract_table_value(document, header_text) {
            Ok(value) => {
                let trimmed = value.trim();
                if trimmed.is_empty() || trimmed == "&nbsp;" {
                    None
                } else {
                    Some(trimmed.to_string())
                }
            }
            Err(_) => None,
        }
    }

    /// アクティブラーニング情報を抽出
    fn extract_active_learning(&self, document: &Html) -> Result<ActiveLearningInfo, ParseError> {
        let mut active_learning = ActiveLearningInfo::default();

        // 各アクティブラーニング項目を個別に抽出
        active_learning.discussion = self.check_active_learning_item(document, "ディスカッション");
        active_learning.debate = self.check_active_learning_item(document, "ディベート");
        active_learning.group_work = self.check_active_learning_item(document, "グループワーク");
        active_learning.presentation =
            self.check_active_learning_item(document, "プレゼンテーション");
        active_learning.practical_training = self.check_active_learning_item(document, "実習");
        active_learning.field_work = self.check_active_learning_item(document, "フィールドワーク");
        active_learning.other_problem_solving_learning =
            self.check_active_learning_item(document, "その他課題解決型学習");

        Ok(active_learning)
    }

    /// 特定のアクティブラーニング項目が有効かどうかをチェック
    fn check_active_learning_item(&self, document: &Html, item_name: &str) -> bool {
        let tr_selector = Selector::parse("tr").unwrap();
        let th_selector = Selector::parse("th").unwrap();
        let td_selector = Selector::parse("td").unwrap();

        for tr_element in document.select(&tr_selector) {
            let th_elements: Vec<_> = tr_element.select(&th_selector).collect();
            let td_elements: Vec<_> = tr_element.select(&td_selector).collect();

            // 大きなテーブル行は無視（すべてのデータが含まれている行を避けるため）
            if th_elements.len() > 10 || td_elements.len() > 10 {
                continue;
            }

            // この行にアクティブラーニング項目のTHがあるかチェック
            for (th_index, th_element) in th_elements.iter().enumerate() {
                let th_text = th_element.text().collect::<String>().trim().to_string();
                if th_text == item_name {
                    // 対応するTDを探して◎が含まれているかチェック
                    if let Some(td_text) =
                        self.find_corresponding_td(&th_elements, &td_elements, th_index)
                    {
                        return td_text.contains("◎");
                    }
                    break;
                }
            }
        }

        false
    }

    /// 授業計画詳細を抽出
    fn extract_lesson_plan_details(
        &self,
        document: &Html,
    ) -> Result<Vec<LessonPlanItem>, ParseError> {
        let mut lesson_plans = Vec::new();
        let tr_selector = Selector::parse("tr")
            .map_err(|e| ParseError::selector_creation_failed("tr", &e.to_string()))?;

        for tr_element in document.select(&tr_selector) {
            let tr_html = tr_element.html();

            // 「第X回」を含む行を探す
            if tr_html.contains("第") && tr_html.contains("回") {
                let th_selector = Selector::parse("th")
                    .map_err(|e| ParseError::selector_creation_failed("th", &e.to_string()))?;
                let td_selector = Selector::parse("td")
                    .map_err(|e| ParseError::selector_creation_failed("td", &e.to_string()))?;

                let tr_doc = Html::parse_fragment(&tr_html);
                let mut session_number = String::new();
                let mut lesson_plan = String::new();
                let mut outside_class_tasks = String::new();

                // セッション番号を抽出
                for th_element in tr_doc.select(&th_selector) {
                    let th_text = th_element.text().collect::<String>().trim().to_string();
                    if th_text.contains("第") && th_text.contains("回") {
                        session_number = th_text;
                        break;
                    }
                }

                // TDから授業計画と課題を抽出
                let td_elements: Vec<_> = tr_doc.select(&td_selector).collect();
                if td_elements.len() >= 2 {
                    lesson_plan = td_elements[0].text().collect::<String>().trim().to_string();
                    outside_class_tasks =
                        td_elements[1].text().collect::<String>().trim().to_string();
                }

                if !session_number.is_empty() {
                    lesson_plans.push(LessonPlanItem {
                        session_number,
                        lesson_plan,
                        outside_class_tasks,
                    });
                }
            }
        }

        Ok(lesson_plans)
    }
}

impl PageParser<LessonInfo> for SyllabusViewParser {
    const PAGE_TYPE: &'static str = "syllabus_view";

    fn parse_document(&self, document: &Html) -> Result<LessonInfo, ParseError> {
        // 基本情報の抽出
        let lesson_code_str = self.extract_table_value(document, "授業コード")?;
        let lesson_code = lesson_code_str
            .trim()
            .parse::<u32>()
            .map_err(|_| ParseError::data_parsing_failed("lesson_code", &lesson_code_str))?;

        let assigned_grade_str = self.extract_table_value(document, "配当学年")?;
        let assigned_grade = assigned_grade_str
            .trim()
            .parse::<u32>()
            .map_err(|_| ParseError::data_parsing_failed("assigned_grade", &assigned_grade_str))?;

        let credits_str = self.extract_table_value(document, "単位数")?;
        let credits = credits_str
            .trim()
            .parse::<u32>()
            .map_err(|_| ParseError::data_parsing_failed("credits", &credits_str))?;

        let subject_name = self.extract_table_value(document, "科目名")?;
        let academic_year_semester = self.extract_table_value(document, "年度学期")?;
        let day_period = self.extract_table_value(document, "曜日時限")?;
        let target_department = self.extract_table_value(document, "対象学科")?;
        let subject_category = self.extract_table_value(document, "科目区分")?;
        let required_elective_distinction = self.extract_table_value(document, "必選の別")?;
        let instructor = self.extract_table_value(document, "担当者")?;

        // オプション項目の抽出
        let omnibus = self.extract_optional_value(document, "オムニバス");
        let course = self.extract_optional_value(document, "コース");
        let classroom = self.extract_optional_value(document, "教室");
        let industry_professional_led_class =
            self.extract_optional_value(document, "実務家教員担当授業");
        let class_objectives_and_approach =
            self.extract_optional_value(document, "授業の目的と進め方");

        // 達成目標の抽出
        let achievement_goal_1 = self.extract_optional_value(document, "達成目標１");
        let achievement_goal_2 = self.extract_optional_value(document, "達成目標２");
        let achievement_goal_3 = self.extract_optional_value(document, "達成目標３");
        let achievement_goal_4 = self.extract_optional_value(document, "達成目標４");
        let achievement_goal_5 = self.extract_optional_value(document, "達成目標５");
        let achievement_goal_6 = self.extract_optional_value(document, "達成目標６");
        let achievement_goal_7 = self.extract_optional_value(document, "達成目標７");

        // その他の情報
        let feedback_on_assignments =
            self.extract_optional_value(document, "課題等に対するフィードバック");
        let evaluation_methods_and_criteria =
            self.extract_optional_value(document, "評価方法と基準");
        let textbook = self.extract_optional_value(document, "テキスト");
        let reference_books = self.extract_optional_value(document, "参考図書");
        let subject_positioning =
            self.extract_optional_value(document, "科目の位置づけ（学習・教育目標との対応）");
        let preparation_before_registration =
            self.extract_optional_value(document, "履修登録前の準備");

        // アクティブラーニング情報の抽出
        let active_learning = self.extract_active_learning(document)?;

        // 授業計画詳細の抽出
        let lesson_plan_details = self.extract_lesson_plan_details(document)?;

        Ok(LessonInfo {
            lesson_code,
            omnibus,
            subject_name,
            assigned_grade,
            credits,
            academic_year_semester,
            day_period,
            target_department,
            course,
            subject_category,
            required_elective_distinction,
            instructor,
            classroom,
            industry_professional_led_class,
            class_objectives_and_approach,
            achievement_goal_1,
            achievement_goal_2,
            achievement_goal_3,
            achievement_goal_4,
            achievement_goal_5,
            achievement_goal_6,
            achievement_goal_7,
            active_learning,
            lesson_plan_details,
            feedback_on_assignments,
            evaluation_methods_and_criteria,
            textbook,
            reference_books,
            subject_positioning,
            preparation_before_registration,
        })
    }
}
