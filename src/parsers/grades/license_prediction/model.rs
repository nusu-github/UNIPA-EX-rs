//! 免許取得見込判定のデータ構造（プレースホルダ実装）

use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// 免許取得見込判定データを表す構造体。
///
/// 注意: この構造体は現在プレースホルダ実装です。
/// 実際の要件に応じて構造を変更する必要があります。
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct LicensePredictionData {
    /// 免許の種類
    pub license_type: String,
    /// 取得見込判定結果
    pub prediction_result: String,
    /// 不足要件の詳細
    pub missing_requirements: Vec<String>,
}
