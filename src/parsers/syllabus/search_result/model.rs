//! シラバス検索結果のデータ構造
//!
//! UNIVERSAL PASSPORT EXのシラバス検索結果ページから取得できる情報を構造化して表現します。
//! 検索条件、結果メタデータ、科目一覧、ページネーション情報を含みます。

use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// シラバス検索結果のページ全体を表現する構造体
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SyllabusSearchResultPage {
    /// 検索条件の表示情報
    pub search_conditions: SearchConditions,
    /// 検索結果のメタ情報
    pub result_metadata: ResultMetadata,
    /// 検索結果の科目一覧
    pub course_entries: Vec<CourseEntry>,
    /// ページネーション情報
    pub pagination: PaginationInfo,
    /// フォーム情報（戻るボタンや隠しフィールド用）
    pub form_info: FormInfo,
}

/// 検索条件の表示情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SearchConditions {
    /// 開講年度／学期
    pub academic_year_semester: String,
    /// 科目名
    pub subject_name: Option<String>,
    /// 学科・コース／専攻
    pub department_course: Option<String>,
}

/// 検索結果のメタ情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ResultMetadata {
    /// 総件数
    pub total_count: u32,
    /// 現在のページ番号（1から開始）
    pub current_page: u32,
    /// 総ページ数
    pub total_pages: u32,
}

/// 個別の科目エントリ
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CourseEntry {
    /// 開講曜日
    pub schedule_day_period: String,
    /// 科目コードと科目名
    pub course_code_and_name: String,
    /// 教員氏名
    pub instructor_names: String,
    /// 開講区分
    pub course_type: String,
    /// 学年
    pub target_grade: Option<String>,
    /// 開講学期
    pub semester: String,
    /// 単位数
    pub credits: String,
    /// シラバス詳細へのリンク情報
    pub syllabus_link: SyllabusLinkInfo,
}

/// シラバス詳細へのリンク情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SyllabusLinkInfo {
    /// リンクのID（例：「form1:htmlKekkatable:0:edit」）
    pub link_id: String,
    /// JavaScriptのonclick属性値
    pub onclick_action: String,
    /// リンクが有効かどうか
    pub is_active: bool,
}

/// ページネーション情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PaginationInfo {
    /// 先頭ページへのボタン状態
    pub first_button_enabled: bool,
    /// 前ページへのボタン状態
    pub previous_button_enabled: bool,
    /// 次ページへのボタン状態
    pub next_button_enabled: bool,
    /// 最終ページへのボタン状態
    pub last_button_enabled: bool,
    /// ページ表示テキスト（例：「1/1 ページ」）
    pub page_display_text: String,
    /// 現在のページ番号（強調表示用）
    pub current_page_number: u32,
}

/// フォーム情報（戻るボタンや隠しフィールド用）
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FormInfo {
    /// フォームのaction属性値
    pub form_action: String,
    /// フォームのmethod属性値
    pub form_method: String,
    /// フォームのenctype属性値
    pub form_enctype: String,
    /// 隠しフィールドの値
    pub hidden_fields: Vec<HiddenField>,
}

/// 隠しフィールド情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct HiddenField {
    /// フィールド名
    pub field_name: String,
    /// フィールド値
    pub field_value: String,
}
