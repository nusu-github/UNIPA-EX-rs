//! 課題提出一覧のデータ構造
//!
//! UNIVERSAL PASSPORT EXの課題提出ページから取得できる課題情報を構造化して表現します。
//! 提出期限、提出状況、課題内容などの詳細情報を含みます。

use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// 課題提出一覧全体を表現する構造体
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct AssignmentList {
    /// 課題一覧
    pub assignments: Vec<Assignment>,
    /// ページネーション情報
    pub pagination: Option<PaginationInfo>,
}

/// 個別の課題情報を表現する構造体
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Assignment {
    /// 科目名
    pub subject_name: String,
    /// 課題タイトル
    pub assignment_title: String,
    /// 提出期限（例：「2025-06-15 23:59」）
    pub due_date: String,
    /// 提出状況
    pub submission_status: SubmissionStatus,
    /// 課題の詳細説明
    pub description: Option<String>,
    /// ファイル添付の有無
    pub has_attachment: bool,
    /// 提出済みファイル名（提出済みの場合）
    pub submitted_file_name: Option<String>,
    /// 提出日時（提出済みの場合）
    pub submission_date: Option<String>,
    /// 教員からのコメント（評価済みの場合）
    pub teacher_comment: Option<String>,
    /// 評価点（評価済みの場合）
    pub score: Option<u32>,
}

/// 提出状況を表現する列挙型
#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum SubmissionStatus {
    /// 未提出
    NotSubmitted,
    /// 提出済み
    Submitted,
    /// 期限切れ（未提出）
    Overdue,
    /// 評価済み
    Evaluated,
}

impl Default for SubmissionStatus {
    fn default() -> Self {
        Self::NotSubmitted
    }
}

/// ページネーション情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PaginationInfo {
    /// 現在のページ番号
    pub current_page: u32,
    /// 総ページ数
    pub total_pages: u32,
    /// 前のページがあるか
    pub has_previous: bool,
    /// 次のページがあるか
    pub has_next: bool,
}
