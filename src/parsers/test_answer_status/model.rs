//! テスト解答状況のデータ構造
//!
//! UNIVERSAL PASSPORT EXのテスト解答一覧ページから取得できる情報を構造化して表現します。
//! 未実施、実施中、実施済の各状態のテスト情報を含みます。

use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// テスト解答状況の全体情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TestAnswerStatus {
    /// 年度
    pub academic_year: String,
    /// タブ情報（未実施、実施中、実施済の件数）
    pub tab_info: TabInfo,
    /// 現在表示中のタブ
    pub current_tab: TestStatus,
    /// テスト一覧
    pub tests: Vec<TestItem>,
}

/// タブ情報（各状態の件数）
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TabInfo {
    /// 未実施のテスト件数
    pub not_implemented_count: u32,
    /// 実施中のテスト件数
    pub in_progress_count: u32,
    /// 実施済のテスト件数
    pub completed_count: u32,
}

/// テストの実施状況
#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum TestStatus {
    /// 未実施
    NotImplemented,
    /// 実施中
    InProgress,
    /// 実施済
    Completed,
}

impl Default for TestStatus {
    fn default() -> Self {
        Self::NotImplemented
    }
}

/// テスト項目の情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TestItem {
    /// テストタイトル
    pub title: String,
    /// 授業名
    pub course_name: String,
    /// 状態固有の詳細情報
    pub details: TestDetails,
}

/// テストの詳細情報（状態によって異なる）
#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum TestDetails {
    /// 未実施テストの詳細
    NotImplemented(NotImplementedDetails),
    /// 実施中テストの詳細
    InProgress(InProgressDetails),
    /// 実施済テストの詳細
    Completed(CompletedDetails),
}

impl Default for TestDetails {
    fn default() -> Self {
        Self::NotImplemented(NotImplementedDetails::default())
    }
}

/// 未実施テストの詳細情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct NotImplementedDetails {
    /// 開始日
    pub start_date: Option<String>,
    /// 終了日
    pub end_date: Option<String>,
    /// 制限時間（分）
    pub time_limit_minutes: Option<u32>,
    /// 再解答可能かどうか
    pub can_reanswer: bool,
}

/// 実施中テストの詳細情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct InProgressDetails {
    /// 実施回数
    pub attempt_count: u32,
    /// 最新実施日
    pub latest_attempt_date: Option<String>,
    /// 実施時間（分）
    pub duration_minutes: Option<u32>,
    /// 点数
    pub score: Option<String>,
}

/// 実施済テストの詳細情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CompletedDetails {
    /// 実施回数
    pub attempt_count: u32,
    /// 最新実施日
    pub latest_attempt_date: Option<String>,
    /// 実施時間（分）
    pub duration_minutes: Option<u32>,
    /// 点数
    pub score: Option<String>,
}
