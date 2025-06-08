//! 卒業見込判定のデータ構造

use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// 卒業見込判定における要件不足項目を表す構造体。
///
/// 各フィールドは、要件の具体的な不足内容と、その不足量を明確に示します。
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct YokenFusokuItem {
    /// 要件のコード。
    pub joken_code: String,
    /// 要件を特定するための要素番号。
    pub yoso_number: i32,
    /// 要件不足の具体的なメッセージ。
    pub fusoku_message: String,
    /// 要件の不足量を示す文字列。
    pub fusoku_ryo: String,
}

/// 卒業見込判定の結果全体を表す構造体。
///
/// 判定メッセージと、複数の要件不足項目リストを含みます。
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SotsugyoMikonHanteiKekka {
    /// 卒業見込判定のメッセージ。
    pub hantei_message: String,
    /// 要件不足項目のリスト。
    /// 不足がない場合は空のベクターとなります。
    pub fusoku_items: Vec<YokenFusokuItem>,
}
