//! 授業評価関連のデータモデルを定義します。
//!
//! このモジュールは、授業評価回答に関するデータを構造化します。
//! `scraper` クレートからパースされたHTML要素を保持するための構造体や、
//! そのデータを効率的に管理するためのDTO (Data Transfer Object) を含みます。

use serde::{Deserialize, Serialize};

/// 授業評価回答のデータを表す構造体
///
/// この構造体は、授業評価の回答から抽出される主要な情報を保持します。
/// 将来的には、評価項目、回答内容、提出日時などの詳細が追加される予定です。
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CourseEvaluation {
    // プレースホルダ: 授業評価回答に関するフィールドをここに追加します。
    // 例: pub id: String,
    // 例: pub course_name: String,
    // 例: pub status: String,
}

impl CourseEvaluation {
    /// 新しい `CourseEvaluation` インスタンスを作成します。
    pub fn new() -> Self {
        CourseEvaluation {}
    }
}

impl Default for CourseEvaluation {
    fn default() -> Self {
        Self::new()
    }
}
