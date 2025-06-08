//! 学籍情報照会のデータ構造
//!
//! UNIVERSAL PASSPORT EXの学籍情報照会ページから取得できる学生の基本情報、
//! 所属情報、指導教員情報、学籍変更履歴を構造化して表現します。

use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// 学生情報全体を表現する構造体
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct StudentInfo {
    /// 基本情報セクション
    pub basic_info: BasicInfo,
    /// 所属情報セクション
    pub affiliation_info: AffiliationInfo,
    /// 指導教員情報セクション
    pub advisor_info: AdvisorInfo,
    /// 学籍変更情報セクション
    pub status_change_info: StatusChangeInfo,
}

/// 学生の基本情報を表現する構造体
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct BasicInfo {
    /// 学籍番号
    pub student_id: String,
    /// 学生氏名
    pub student_name: String,
    /// 学生氏名（カタカナ）
    pub kana_name: String,
    /// 性別
    pub gender: String,
    /// 生年月日（例：「2003年01月10日」→「2003-01-10」）
    pub date_of_birth: String,
    /// 国籍（記載がない場合はNone）
    pub nationality: Option<String>,
    /// PCメールアドレス（記載がない場合はNone）
    pub pc_email_address: Option<String>,
    /// 入学区分
    pub enrollment_type: String,
    /// 学生身分種別
    pub student_status_type: String,
    /// 入学年度
    pub enrollment_year: u32,
    /// 入学期番号
    pub enrollment_term_no: u32,
    /// カリキュラム対象年度
    pub curriculum_target_year: u32,
    /// カリキュラム対象期
    pub curriculum_target_term: u32,
    /// 入学年月日（例：「2021年04月01日」→「2021-04-01」）
    pub enrollment_date: String,
    /// 退学年月日（記載がない場合はNone）
    pub withdrawal_date: Option<String>,
    /// 卒業予定年月（例：「2026年03月」→「2026-03」）
    pub expected_graduation_month_year: String,
    /// 修了年月日（記載がない場合はNone）
    pub completion_date: Option<String>,
}

/// 学生の所属情報を表現する構造体
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct AffiliationInfo {
    /// 所属学部・組織
    pub affiliated_department_organization: String,
    /// カリキュラム学部・組織
    pub curriculum_department_organization: String,
    /// 学年
    pub grade_level: u32,
    /// セメスター
    pub semester: u32,
    /// 主専攻コース（記載がない場合はNone）
    pub major_course: Option<String>,
    /// クラス種別＋クラス（例：「2クラス割　A<BR>3クラス割　Ⅰ<BR>」→複数行データ）
    pub class_type_class: String,
}

/// 学生の指導教員情報を表現する構造体
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct AdvisorInfo {
    /// 指導教員氏名
    pub advisor_name: String,
    /// 指導開始日（例：「2025年04月01日」→「2025-04-01」）
    pub advisor_start_date: String,
    /// 指導終了日（例：「2026年03月31日」→「2026-03-31」）
    pub advisor_end_date: String,
}

/// 学生の学籍変更情報を表現する構造体
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct StatusChangeInfo {
    /// 学籍変更履歴のリスト（例：「セメスター進行（2025年04月01日）」→「Semester progression (2025-04-01)」）
    pub academic_status_history: Vec<String>,
}
