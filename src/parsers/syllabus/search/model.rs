//! シラバス検索フォームのデータモデル
//!
//! UNIVERSAL PASSPORT EXのシラバス検索条件を表現するデータ構造を定義します。

use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// シラバス検索フォームの構造体
///
/// UNIVERSAL PASSPORT EXのシラバス検索で使用可能な全ての検索条件を含みます。
/// すべてのフィールドはオプショナルで、指定されない場合は検索条件から除外されます。
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SyllabusSearchForm {
    /// 管理部署名
    pub kanri_bsyo_name: Option<String>,
    /// 年度
    pub nendo: Option<u32>,
    /// 学期
    pub gakki_no: Option<String>,
    /// 科目区分
    pub kamok_jugyo: Option<String>,
    /// 科目名
    pub kamok_name: Option<String>,
    /// 担当教員
    pub kyoin_simei: Option<String>,
    /// 学科・コース／専攻
    pub gakka: Option<String>,
    /// 学年
    pub gakunen: Option<String>,
    /// 曜日
    pub yobi: Option<String>,
    /// 時限
    pub jigen: Option<String>,
    /// 集中講義など
    pub syutyu: Option<bool>,
    /// キーワード
    pub keyword: Option<String>,
    /// 識別区分
    pub shikibetsu_kbn: Option<u32>,
    /// 管理番号
    pub kanri_no: Option<u32>,
}
