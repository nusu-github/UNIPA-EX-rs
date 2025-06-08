//! 教室予約状況のデータ構造
//!
//! UNIVERSAL PASSPORT EXの教室予約状況ページ（Ksc00101A）から取得できる情報を構造化して表現します。
//! 検索条件、予約状況テーブル、各種ポップアップ詳細などの情報を含みます。

use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// 教室予約状況ページ全体から抽出されるデータを格納するメイン構造体
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ClassroomReservationStatus {
    /// ページ上部の検索条件
    pub search_params: SearchParams,
    /// 教室予約状況表示テーブルのデータ
    pub reservation_table_data: Vec<ClassroomReservationTableData>,
    /// 予約詳細ポップアップデータ（存在する場合）
    pub popup_reservation_detail: Option<PopupReservationDetail>,
    /// 授業詳細ポップアップデータ（存在する場合）
    pub popup_class_detail: Option<PopupClassDetail>,
    /// 重複授業詳細ポップアップデータ（存在する場合）
    pub popup_duplicate_class_detail: Option<PopupDuplicateClassDetail>,
    /// 教室詳細ポップアップデータ（存在する場合）
    pub popup_classroom_detail: Option<PopupClassroomDetail>,
}

/// ページ上部の検索条件を格納する構造体
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SearchParams {
    /// 検索対象年度
    pub academic_year: String,
    /// 検索対象学期
    pub semester: String,
    /// 検索対象曜日
    pub day_of_week: String,
    /// 検索対象時限
    pub period: String,
    /// 検索対象棟
    pub building: String,
    /// 検索対象教室
    pub classroom: String,
    /// 検索対象科目名
    pub subject_name: String,
    /// 検索対象担当教員名
    pub instructor_name: String,
}

/// 教室予約状況表示テーブルの各行のデータを格納する構造体
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ClassroomReservationTableData {
    /// 時限
    pub period: String,
    /// 月曜日の予約状況
    pub monday: ClassroomReservationCell,
    /// 火曜日の予約状況
    pub tuesday: ClassroomReservationCell,
    /// 水曜日の予約状況
    pub wednesday: ClassroomReservationCell,
    /// 木曜日の予約状況
    pub thursday: ClassroomReservationCell,
    /// 金曜日の予約状況
    pub friday: ClassroomReservationCell,
    /// 土曜日の予約状況
    pub saturday: ClassroomReservationCell,
    /// 日曜日の予約状況
    pub sunday: ClassroomReservationCell,
}

/// 教室予約状況テーブルの各セル（曜日×時限）の情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ClassroomReservationCell {
    /// 予約状況のタイプ
    pub reservation_type: ReservationType,
    /// 科目名（予約がある場合）
    pub subject_name: Option<String>,
    /// 担当教員名（予約がある場合）
    pub instructor_name: Option<String>,
    /// 教室名
    pub classroom_name: Option<String>,
    /// 詳細情報へのリンクURL（存在する場合）
    pub detail_link_url: Option<String>,
    /// セルの背景色やスタイル情報
    pub cell_style: Option<String>,
}

/// 予約状況のタイプを表現する列挙型
#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum ReservationType {
    /// 予約なし（空き状況）
    Available,
    /// 通常授業で予約済み
    RegularClass,
    /// 集中講義で予約済み
    IntensiveCourse,
    /// その他の予約
    Other,
    /// 重複予約
    Duplicate,
    /// 利用不可
    Unavailable,
}

impl Default for ReservationType {
    fn default() -> Self {
        Self::Available
    }
}

/// 予約詳細ポップアップから抽出されるデータを格納する構造体
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PopupReservationDetail {
    /// 予約ID
    pub reservation_id: String,
    /// 科目名
    pub subject_name: String,
    /// 科目コード
    pub subject_code: String,
    /// 担当教員名
    pub instructor_name: String,
    /// 教室名
    pub classroom_name: String,
    /// 開始日時（ISO 8601形式: YYYY-MM-DDTHH:MM:SS+hh:mm）
    pub start_datetime: String,
    /// 終了日時（ISO 8601形式: YYYY-MM-DDTHH:MM:SS+hh:mm）
    pub end_datetime: String,
    /// 履修者数
    pub enrollment_count: Option<u32>,
    /// 備考
    pub notes: Option<String>,
}

/// 授業詳細ポップアップから抽出されるデータを格納する構造体
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PopupClassDetail {
    /// 授業ID
    pub class_id: String,
    /// 科目名
    pub subject_name: String,
    /// 科目コード
    pub subject_code: String,
    /// 担当教員名
    pub instructor_name: String,
    /// 単位数
    pub credit_count: u32,
    /// 履修者数
    pub enrollment_count: u32,
    /// 開講学期
    pub semester: String,
    /// 曜日・時限
    pub schedule: String,
    /// 教室名
    pub classroom_name: String,
    /// 授業概要
    pub course_description: Option<String>,
}

/// 重複授業詳細ポップアップから抽出されるデータを格納する構造体
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PopupDuplicateClassDetail {
    /// 重複している授業のリスト
    pub conflicting_classes: Vec<ConflictingClass>,
    /// 重複の詳細説明
    pub conflict_description: Option<String>,
    /// 重複解決の提案
    pub resolution_suggestion: Option<String>,
}

/// 重複している個別の授業情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ConflictingClass {
    /// 授業ID
    pub class_id: String,
    /// 科目名
    pub subject_name: String,
    /// 担当教員名
    pub instructor_name: String,
    /// 履修者数
    pub enrollment_count: u32,
    /// 優先度
    pub priority: Option<String>,
}

/// 教室詳細ポップアップから抽出されるデータを格納する構造体
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PopupClassroomDetail {
    /// 教室ID
    pub classroom_id: String,
    /// 教室名
    pub classroom_name: String,
    /// 棟名
    pub building_name: String,
    /// フロア
    pub floor: String,
    /// 収容人数
    pub capacity: u32,
    /// 設備情報
    pub equipment: Vec<String>,
    /// 利用可能時間帯
    pub available_hours: String,
    /// 備考
    pub notes: Option<String>,
}
