//! お知らせ詳細のデータ構造
//!
//! UNIVERSAL PASSPORT EXのお知らせ詳細ポップアップ画面に表示される情報を構造化して表現します。
//! タイトル、送信者、本文、添付ファイルなどの詳細情報を含みます。

use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// お知らせ詳細の情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct NotificationDetail {
    /// タイトル
    pub title: String,
    /// 送信者
    pub from: String,
    /// メイン本文
    pub main_text: String,
    /// 添付ファイルリスト
    pub attachments: Vec<AttachmentFile>,
    /// 閉じるボタン
    pub close_button: String,
}

/// 添付ファイル情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct AttachmentFile {
    /// ファイル名
    pub file_name: String,
    /// ファイルサイズ
    pub file_size: String,
    /// ダウンロードボタンID
    pub download_button_id: String,
}