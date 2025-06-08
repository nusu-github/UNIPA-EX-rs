//! アンケート一覧のデータ構造
//!
//! UNIVERSAL PASSPORT EXのアンケート一覧ページから取得できる情報を構造化して表現します。
//! 回答可能なアンケート項目、期限、回答状況などの情報を含みます。

use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// アンケート一覧の全体情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct QuestionnaireList {
    /// アンケート項目のリスト
    pub questionnaires: Vec<QuestionnaireItem>,
    /// ページネーション情報（ある場合）
    pub pagination: Option<PaginationInfo>,
}

/// 個別のアンケート項目
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct QuestionnaireItem {
    /// アンケートタイトル
    pub title: String,
    /// 対象科目名（ある場合）
    pub subject_name: Option<String>,
    /// 担当教員名（ある場合）
    pub instructor_name: Option<String>,
    /// 回答期限
    pub deadline: String,
    /// 回答状況
    pub response_status: ResponseStatus,
    /// アンケート詳細へのリンク情報
    pub questionnaire_link: QuestionnaireLink,
}

/// 回答状況の種類
#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum ResponseStatus {
    /// 未回答
    NotAnswered,
    /// 回答済み
    Answered,
    /// 期限切れ
    Expired,
}

impl Default for ResponseStatus {
    fn default() -> Self {
        Self::NotAnswered
    }
}

/// アンケート詳細へのリンク情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct QuestionnaireLink {
    /// リンクのURL
    pub url: String,
    /// リンクパラメータ
    pub parameters: Vec<LinkParameter>,
    /// リンクが有効かどうか
    pub is_active: bool,
}

/// リンクパラメータ
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct LinkParameter {
    /// パラメータ名
    pub name: String,
    /// パラメータ値
    pub value: String,
}

/// ページネーション情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PaginationInfo {
    /// 現在のページ番号
    pub current_page: u32,
    /// 総ページ数
    pub total_pages: u32,
    /// 総件数
    pub total_count: u32,
    /// 次ページへのリンク（ある場合）
    pub next_page_link: Option<String>,
    /// 前ページへのリンク（ある場合）
    pub previous_page_link: Option<String>,
}
