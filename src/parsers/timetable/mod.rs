//! 時間割関連のページパーサーとデータ構造
//!
//! このモジュールは、UNIVERSAL PASSPORT EXの時間割機能に関連する各種ページを解析し、
//! 構造化されたデータとして提供します。学生時間割、教員時間割、
//! 学科開講一覧の3つの主要な機能をサポートします。

/// 学科開講一覧（授業提供側の視点）
pub mod course_timetable;

/// 学生時間割（履修者側の視点）
pub mod student_timetable;

/// 教員時間割（担当者側の視点）
pub mod teacher_timetable;

/// 一覧表示 1 行ぶんの必須情報
pub struct BaseClassInfo {
    /// 曜日と時限（例: "月 3"）
    pub day_and_period: String,
    /// 授業コード
    pub class_code: String,
    /// 科目名
    pub subject_name: String,
    /// 担当教員名
    pub teacher_name: String,
    /// 教室名（空欄の場合はNone）
    pub classroom: Option<String>,
}

/// カレンダー 1 日分（どのエントリ型でも使えるよう型パラメータ E）
pub struct CalendarDay<E> {
    /// 曜日（"月", "火" など）
    pub day_of_week: String,
    /// 時限ごとのセル（1〜6限＋昼休み）
    pub periods: Vec<Option<CalendarCell<E>>>,
}

/// カレンダーの各セル内容
pub enum CalendarCell<E> {
    /// 昼休み
    LunchBreak,
    /// 授業
    Class(E),
}

/// 表示形式
pub enum DisplayFormat {
    /// カレンダー表示
    Calendar,
    /// 一覧表示
    List,
}

/// スケジュールデータ（エントリ型を総称化）
pub enum ScheduleData<E> {
    /// 一覧形式のデータ
    List(Vec<E>),
    /// カレンダー形式のデータ
    Calendar(Vec<CalendarDay<E>>),
}

// =======================
// 学生時間割用（Kma00401A）

/// 学生時間割の一覧表示 1 行（単位数やエラー情報付き）
pub struct StudentClassEntry {
    /// 基本授業情報
    pub base: BaseClassInfo,
    /// 単位数
    pub credits: u32,
    /// エラーメッセージ（ある場合）
    pub error: Option<String>,
}

/// 集中講義・実習（学生用、単位とエラー情報付き）
pub struct StudentIrregularEntry {
    /// 基本授業情報
    pub base: BaseClassInfo,
    /// 単位数
    pub credits: u32,
    /// エラーメッセージ（ある場合）
    pub error: Option<String>,
}

/// 単位取得状況の全体構造
pub struct CreditStatus {
    /// 教育課程の単位取得状況
    pub education_program: EducationProgramCreditStatus,
    /// 専門科目の単位取得状況
    pub specialized_subject: SpecializedSubjectCreditStatus,
}

/// 教育課程の単位取得状況詳細
pub struct EducationProgramCreditStatus {
    /// 卒業要件単位
    pub graduation_requirement_credits: EducationProgramCreditDetails,
    /// 修得済単位
    pub acquired_credits: EducationProgramCreditDetails,
    /// 履修中単位
    pub registered_credits: EducationProgramCreditDetails,
    /// 合計単位
    pub total_credits: EducationProgramCreditDetails,
}

/// 教育課程の各カテゴリの単位詳細
pub struct EducationProgramCreditDetails {
    /// 総計
    pub total: u32,
    /// 教育課程
    pub educational_curriculum: u32,
    /// 共通教育
    pub general_education: u32,
    /// 学基キャリア合計
    pub academic_basic_career_total: u32,
    /// 学習基盤
    pub learning_foundation: u32,
    /// キャリア
    pub career: u32,
    /// その他
    pub other: u32,
    /// 教養コア
    pub liberal_arts_core: u32,
    /// 教養アド
    pub liberal_arts_advanced: u32,
    /// 言語系合計
    pub language_system_total: u32,
    /// 言語系必修科目
    pub language_required: u32,
    /// 言語系選択科目
    pub language_elective: u32,
    /// 理数系合計
    pub science_system_total: u32,
    /// 理数系必修科目
    pub science_required: u32,
    /// 理数系選択科目
    pub science_elective: u32,
    /// 環境系
    pub environment_system: u32,
}

/// 専門科目の単位取得状況詳細
pub struct SpecializedSubjectCreditStatus {
    /// 卒業要件単位
    pub graduation_requirement_credits: SpecializedSubjectCreditDetails,
    /// 修得済単位
    pub acquired_credits: SpecializedSubjectCreditDetails,
    /// 履修中単位
    pub registered_credits: SpecializedSubjectCreditDetails,
    /// 合計単位
    pub total_credits: SpecializedSubjectCreditDetails,
}

/// 専門科目の各カテゴリの単位詳細
pub struct SpecializedSubjectCreditDetails {
    /// 総計
    pub total: u32,
    /// 単位認定
    pub credit_recognition: u32,
    /// 専門科目
    pub specialized_subject: u32,
    /// 専門教育合計
    pub specialized_education_total: u32,
    /// 専門教育必修科目
    pub specialized_education_required: u32,
    /// 専門教育選択科目
    pub specialized_education_elective: u32,
    /// 学科専門合計
    pub department_specialized_total: u32,
    /// 学科専門必修科目
    pub department_specialized_required: u32,
    /// 学科専門選択科目
    pub department_specialized_elective: u32,
    /// カレッジ
    pub college: u32,
    /// 資格認定
    pub certification: u32,
    /// オープン
    pub open: u32,
    /// 単位互換
    pub credit_exchange: u32,
    /// 教職課程
    pub teacher_training_course: u32,
    /// 自由科目
    pub free_subject: u32,
    /// 総合計
    pub overall_total: u32,
}

/// 学生時間割（Kma00401A）全体
pub struct StudentTimetable {
    /// 開講年度
    pub opening_year: u32,
    /// 学期
    pub semester: String,
    /// 表示形式
    pub display_format: DisplayFormat,
    /// スケジュールデータ
    pub schedule: ScheduleData<StudentClassEntry>,
    /// 集中講義・実習
    pub irregular_classes: Vec<StudentIrregularEntry>,
    /// 単位取得状況
    pub credit_status: CreditStatus,
    /// その他のエラー
    pub other_errors: Vec<OtherError>,
    /// 履修合計単位
    pub total_registered_credits: TotalRegisteredCredits,
}

/// その他のエラー項目
pub struct OtherError {
    /// エラーコード
    pub error_code: String,
    /// 留意No
    pub attention_no: u32,
    /// 内容
    pub content: String,
    /// 不足数（例: "4 単位"）
    pub deficiency_count: String,
}

/// 履修合計単位の情報
pub struct TotalRegisteredCredits {
    /// 履修合計単位数
    pub total_registered_credits_value: u32,
}

// =======================
// 学科開講一覧用（Kma00203A）

/// 提供科目エントリ（単位やエラー情報なし）
pub struct OfferingClassEntry {
    /// 基本授業情報
    pub base: BaseClassInfo,
}

/// 集中講義・実習（提供科目用、最小限の情報）
pub type OfferingIrregularEntry = OfferingClassEntry;

/// 学科開講一覧（Kma00203A）全体
pub struct DepartmentTimetable {
    /// 学生情報ラベル（大学・学部など判別用）
    pub student_info_label: String,
    /// 開講年度
    pub opening_year: u32,
    /// 学期
    pub semester: String,
    /// 表示形式
    pub display_format: DisplayFormat,
    /// スケジュールデータ
    pub schedule: ScheduleData<OfferingClassEntry>,
    /// 集中講義・実習
    pub irregular_classes: Vec<OfferingIrregularEntry>,
}
