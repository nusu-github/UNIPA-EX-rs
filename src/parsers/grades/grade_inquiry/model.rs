//! 成績照会のデータ構造
//!
//! UNIVERSAL PASSPORT EXの成績照会ページから取得できる情報を構造化して表現します。
//! 科目ごとの成績、GPA、単位取得状況などの詳細情報を含みます。

use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// 成績照会の全体情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct GradeInquiry {
    /// 表示パターン
    pub display_pattern: DisplayPattern,
    /// 表示設定
    pub display_settings: DisplaySettings,
    /// 科目リスト
    pub subjects: Vec<Subject>,
    /// GPA得点
    pub gpa_score: f64,
    /// 単位取得状況サマリー
    pub credit_summary: CreditSummary,
}

/// 表示パターン設定
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct DisplayPattern {
    /// 表示タイプ
    pub view_type: ViewType,
}

/// 表示タイプの種類
#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum ViewType {
    /// 通常表示
    Standard,
    /// 年度学期別表示
    BySemester,
}

impl Default for ViewType {
    fn default() -> Self {
        Self::Standard
    }
}

/// 表示設定の詳細
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct DisplaySettings {
    /// 評価名称の表示可否
    pub is_grade_label_visible: bool,
    /// 素点の表示可否
    pub is_numeric_score_visible: bool,
    /// 出席率の表示可否
    pub is_attendance_visible: bool,
    /// 不合格科目の表示可否
    pub is_failed_subjects_visible: bool,
    /// 履修中科目の表示可否
    pub is_current_subjects_visible: bool,
    /// GPAの表示可否
    pub is_gpa_visible: bool,
    /// 単位修得状況の表示可否
    pub is_credit_status_visible: bool,
}

/// 科目情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Subject {
    /// 科目名
    pub name: String,
    /// 単位数
    pub credit_count: Option<u32>,
    /// 成績評価
    pub grade: Option<Grade>,
    /// 素点
    pub numeric_score: Option<u32>,
    /// 履修年度
    pub academic_year: u32,
    /// 履修学期
    pub semester: Semester,
    /// 担当教員名
    pub instructor_name: String,
    /// 現在履修中かどうか
    pub is_currently_enrolled: bool,
    /// 科目カテゴリ
    pub category: SubjectCategory,
}

/// 成績評価の種類
#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Grade {
    /// 秀
    AA,
    /// 優
    A,
    /// 良
    B,
    /// 可
    C,
    /// 不可
    D,
    /// 合格
    Pass,
    /// 評価なし
    NoEvaluation,
}

impl Default for Grade {
    fn default() -> Self {
        Self::NoEvaluation
    }
}

/// 学期の種類
#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Semester {
    /// 春学期
    Spring,
    /// 秋学期
    Fall,
}

impl Default for Semester {
    fn default() -> Self {
        Self::Spring
    }
}

/// 科目カテゴリ情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SubjectCategory {
    /// カリキュラム名
    pub curriculum_name: String,
    /// 大分類名
    pub major_category_name: String,
    /// 中分類名
    pub middle_category_name: String,
    /// 小分類名
    pub sub_category_name: String,
    /// 必修・選択の区分
    pub requirement_type: RequirementType,
    /// 階層レベル
    pub hierarchy_level: u32,
}

/// 必修・選択の区分
#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum RequirementType {
    /// 必修
    Required,
    /// 選択
    Elective,
}

impl Default for RequirementType {
    fn default() -> Self {
        Self::Elective
    }
}

/// 単位取得状況のサマリー
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CreditSummary {
    /// 全体の単位状況
    pub overall: CreditDetails,
    /// 共通教育の単位状況
    pub common_education: CreditDetails,
    /// 専門教育の単位状況
    pub specialized_education: CreditDetails,
    /// カテゴリ別の単位状況
    pub category_breakdown: Vec<CategoryCredits>,
}

/// 単位の詳細情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CreditDetails {
    /// 卒業要件単位数
    pub required_for_graduation: u32,
    /// 修得済み単位数
    pub completed_credits: u32,
    /// 履修中単位数
    pub currently_enrolled_credits: u32,
    /// 合計単位数
    pub total_credits: u32,
}

/// カテゴリ別の単位情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CategoryCredits {
    /// カテゴリ名
    pub category_name: String,
    /// 単位詳細
    pub credit_details: CreditDetails,
}
