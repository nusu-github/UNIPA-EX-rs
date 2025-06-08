//! シラバス詳細表示のデータ構造
//!
//! UNIVERSAL PASSPORT EXのシラバス詳細表示ページから取得できる授業情報を構造化して表現します。
//! 授業の基本情報、目標、計画、評価方法、アクティブラーニング情報等を含みます。

use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// アクティブラーニング情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ActiveLearningInfo {
    /// ディスカッション
    pub discussion: bool,
    /// ディベート
    pub debate: bool,
    /// グループワーク
    pub group_work: bool,
    /// プレゼンテーション
    pub presentation: bool,
    /// 実習
    pub practical_training: bool,
    /// フィールドワーク
    pub field_work: bool,
    /// その他課題解決型学習
    pub other_problem_solving_learning: bool,
}

/// 授業計画項目
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct LessonPlanItem {
    /// 第X回 (例: 第１回)
    pub session_number: String,
    /// 授業計画
    pub lesson_plan: String,
    /// 授業時間外課題（予習および復習を含む）
    pub outside_class_tasks: String,
}

/// シラバス詳細情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct LessonInfo {
    /// 授業コード
    pub lesson_code: u32,
    /// オムニバス
    pub omnibus: Option<String>,
    /// 科目名
    pub subject_name: String,
    /// 配当学年
    pub assigned_grade: u32,
    /// 単位数
    pub credits: u32,
    /// 年度学期
    pub academic_year_semester: String,
    /// 曜日時限
    pub day_period: String,
    /// 対象学科
    pub target_department: String,
    /// コース
    pub course: Option<String>,
    /// 科目区分
    pub subject_category: String,
    /// 必選の別
    pub required_elective_distinction: String,
    /// 担当者
    pub instructor: String,
    /// 教室
    pub classroom: Option<String>,
    /// 実務家教員担当授業
    pub industry_professional_led_class: Option<String>,
    /// 授業の目的と進め方
    pub class_objectives_and_approach: Option<String>,
    /// 達成目標１
    pub achievement_goal_1: Option<String>,
    /// 達成目標２
    pub achievement_goal_2: Option<String>,
    /// 達成目標３
    pub achievement_goal_3: Option<String>,
    /// 達成目標４
    pub achievement_goal_4: Option<String>,
    /// 達成目標５
    pub achievement_goal_5: Option<String>,
    /// 達成目標６
    pub achievement_goal_6: Option<String>,
    /// 達成目標７
    pub achievement_goal_7: Option<String>,
    /// アクティブラーニング
    pub active_learning: ActiveLearningInfo,
    /// 授業計画詳細
    pub lesson_plan_details: Vec<LessonPlanItem>,
    /// 課題等に対するフィードバック
    pub feedback_on_assignments: Option<String>,
    /// 評価方法と基準
    pub evaluation_methods_and_criteria: Option<String>,
    /// テキスト
    pub textbook: Option<String>,
    /// 参考図書
    pub reference_books: Option<String>,
    /// 科目の位置づけ（学習・教育目標との対応）
    pub subject_positioning: Option<String>,
    /// 履修登録前の準備
    pub preparation_before_registration: Option<String>,
}
