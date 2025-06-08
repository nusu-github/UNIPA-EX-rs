//! 進級見込判定のデータ構造

use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// 進級見込判定データを表す構造体。
/// この構造体は、HTMLフォームの隠しフィールドと表示メッセージから抽出された情報を保持します。
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PromotionPredictionData {
    /// フォームに表示される進級見込判定メッセージ。
    pub judgement_message: String,
    /// 最後に検索された学生コード。
    /// 学生を特定するための隠しフィールドとして使用されます。
    pub last_search_student_id: String,
    /// 現在の学年。
    /// フォームの隠しフィールドとして、処理対象の学年を示します。
    pub academic_year: String,
    /// 現在の学期。
    /// フォームの隠しフィールドとして、処理対象の学期を示します。
    pub semester: String,
}
