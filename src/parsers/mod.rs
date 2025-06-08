//! UNIVERSAL PASSPORT EXのHTMLページを解析するパーサーモジュール群
//!
//! このモジュールは、UNIVERSAL PASSPORT EXの各機能ページから情報を抽出するための
//! 専用パーサーを提供します。各サブモジュールは特定のページタイプに
//! 対応しており、構造化されたデータとして情報を返します。

/// 課題提出関連のページパーサー
pub mod assignment_submit;

/// クラスプロファイル（授業詳細）関連のページパーサー
pub mod class_profile;

/// 教室予約状況関連のページパーサー
pub mod classroom_reservation_status;

/// 授業評価関連のページパーサー
pub mod course_evaluation;

/// 成績・単位関連のページパーサー
pub mod grades;

/// ポータル（メイン画面）関連のページパーサー
pub mod portal;

/// アンケート関連のページパーサー
pub mod questionnaire;

/// 学籍情報照会関連のページパーサー
pub mod student_info_inquiry;

/// シラバス関連のページパーサー
pub mod syllabus;

/// テスト解答状況関連のページパーサー
pub mod test_answer_status;

/// 時間割関連のページパーサー（学生・教員・学科開講）
pub mod timetable;

#[cfg(target_arch = "wasm32")]
pub mod wasm;
