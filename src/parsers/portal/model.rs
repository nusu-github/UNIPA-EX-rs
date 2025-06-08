//! ポータル（メイン画面）のデータ構造
//!
//! UNIVERSAL PASSPORT EXのメイン画面に表示される各種情報を構造化して表現します。
//! カレンダー、スケジュール、お気に入りリンク、お知らせなどの
//! 主要コンポーネントを含みます。

use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// ポータル画面全体の情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Portal {
    /// カレンダーコンポーネント
    pub calendar: Calendar,
    /// スケジュールコンポーネント
    pub schedule: Schedule,
    /// お気に入りリンクコンポーネント
    pub favorite_links: FavoriteLinks,
    /// お知らせコンポーネント
    pub notifications: Notifications,
}

/// カレンダーコンポーネント
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Calendar {
    /// 前月ボタン
    pub last_month_button: String,
    /// 次月ボタン
    pub next_month_button: String,
    /// 本日ボタン
    pub current_day_button: String,
    /// 月間スケジュールボタン
    pub month_schedule_button: String,
    /// 年表示
    pub year: String,
    /// 月表示
    pub month: String,
    /// 現在日付（hidden）
    pub current_date: String,
    /// 選択日付（hidden）
    pub selected_day: String,
    /// カレンダーの日付グリッド（週のリストで、各週が日のリストを含む）
    pub days: Vec<Vec<CalendarDay>>,
}

/// カレンダーの1日分の情報
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CalendarDay {
    /// 日付の数値（`&nbsp;` の場合はNone）
    pub day_number: Option<u32>,
    /// その日が今日かどうか（`todayColor` クラスがあるかで判断）
    pub is_today: bool,
    /// その日のリンク先（JavaScript関数呼び出しなど、存在しない場合はNone）
    pub link: Option<String>,
    /// CSSクラス（todayColor, notTodayColor等）
    pub css_classes: Vec<String>,
    /// セルID（form1:Poa00101A:htmlCalendarTable:X:gridY）
    pub cell_id: Option<String>,
}

/// スケジュールコンポーネント
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Schedule {
    /// タイトル
    pub title: String,
    /// 今日の時限割テーブルのエントリリスト
    pub entries: Vec<ScheduleEntry>,
    /// 選択授業コード（hidden）
    pub selected_class_code: String,
}

/// スケジュールの1エントリ
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ScheduleEntry {
    /// 日付
    pub date: String,
    /// 授業内容
    pub class_content: String,
    /// 画像パス（区切り線など、存在しない場合はNone）
    pub image_path: Option<String>,
}

/// お気に入りリンクコンポーネント
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FavoriteLinks {
    /// タイトル
    pub title: String,
    /// 編集ボタン
    pub edit_button: String,
    /// お気に入りリンクのリスト
    pub links: Vec<FavoriteLink>,
}

/// お気に入りリンクの1項目
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FavoriteLink {
    /// リンク名
    pub name: String,
    /// リンクURL（hidden）
    pub url: String,
    /// リンクパラメータ（hidden）
    pub params: String,
    /// リンクメソッド（hidden）
    pub method: String,
}

/// お知らせコンポーネント
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Notifications {
    /// お知らせセクションのリスト
    pub sections: Vec<NotificationSection>,
    /// 全て表示ボタン
    pub all_info_button: String,
}

/// お知らせの1セクション
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct NotificationSection {
    /// ヘッダータイトル
    pub header_title: String,
    /// 関連するコメント（存在しない場合はNone）
    pub comment: Option<String>,
    /// 通知エントリのリスト
    pub entries: Vec<NotificationEntry>,
    /// そのセクションの全件数
    pub total_count: String,
    /// セクション識別子
    pub section_id: String,
    /// 表示モード（"summary" | "all"）
    pub display_mode: String,
    /// 全て表示ボタンの有無
    pub has_all_button: bool,
}

/// お知らせの1エントリ
#[derive(Tsify, Serialize, Deserialize, Clone, Default, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct NotificationEntry {
    /// 既読/未読アイコンの画像パス（存在しない場合はNone）
    pub read_status_image: Option<String>,
    /// 重要度を示すアイコンの画像パス（存在しない場合はNone）
    pub important_status_image: Option<String>,
    /// タイトル（存在しない場合はNone）
    pub title: Option<String>,
    /// 情報源（存在しない場合はNone）
    pub source: Option<String>,
    /// 掲載日
    pub insert_date: String,
} 