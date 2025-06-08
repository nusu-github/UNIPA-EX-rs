//! # Page Parsing のコアトレイト
//!
//! 本モジュールでは、Universal Passport EX 解析のための新しい汎用アーキテクチャに準拠し、
//! すべてのページパーサーが実装すべきコアトレイトを定義しています。

use scraper::Html;

use crate::utils::error::ParseError;

/// 汎用的なパース処理インターフェース
///
/// すべてのページパーサーはこのトレイトを実装し、一貫したパース機能を提供する必要があります。
/// これはすべてのパーサーに共通する基本契約を定義する中核インターフェースです。
pub trait PageParser<T> {
    /// このパーサーが処理するページタイプを識別する
    const PAGE_TYPE: &'static str;

    /// 事前にパース済みのHTMLドキュメントからパースを行う
    ///
    /// 既にパース済みの`Html`ドキュメントがある場合、このメソッドを使用することで
    /// 重複したパース処理を回避できます。
    fn parse_document(&self, document: &Html) -> Result<T, ParseError>;
}

/// セクション特化型パースインターフェース
///
/// ページ内の特定セクション（テーブル、フォーム、ナビゲーション領域など）のパース用。
/// これにより、異なるセクションを独立して処理可能なモジュール型パースを実現します。
pub trait SectionParser<T> {
    /// 特定セクションをパースし、構造化データを返す
    fn parse_section(&self, document: &Html) -> Result<T, ParseError>;

    /// 対象セクションがドキュメント内に存在するかを確認する
    fn section_exists(&self, document: &Html) -> bool;
}

/// データ構築パターンインターフェース
///
/// パース済みHTMLコンテンツから複雑なデータ構造を構築するためのビルダーパターンを実装。
/// パース処理とデータ構築ロジックを分離します。
pub trait DataBuilder<T> {
    /// デフォルト値で新規ビルダーインスタンスを作成する
    fn new() -> Self;

    /// 最終的なデータ構造を構築する
    ///
    /// # エラー処理
    ///
    /// 必要なフィールドが欠落している場合や無効な場合にエラーを返す
    fn build(self) -> Result<T, ParseError>;

    /// 構築したデータオブジェクトの妥当性を検証する
    fn validate(&self, data: &T) -> Result<(), ParseError>;
}
