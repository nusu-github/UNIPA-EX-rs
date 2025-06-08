//! シラバス検索結果パーサーのビルダー
//!
//! ビルダーパターンを使用してシラバス検索結果パーサーを構築します。

use super::parser::SyllabusSearchResultParser;
use crate::utils::error::ParseError;

/// シラバス検索結果パーサービルダー
///
/// ビルダーパターンを使用してSyllabusSearchResultParserを段階的に構築します。
/// 現在は設定オプションは少ないですが、将来の拡張を想定した構造になっています。
#[derive(Default, Debug)]
pub struct SyllabusSearchResultParserBuilder {
    debug_mode: bool,
    strict_mode: bool,
}

impl SyllabusSearchResultParserBuilder {
    /// 新しいビルダーインスタンスを作成
    pub fn new() -> Self {
        Self::default()
    }

    /// デバッグモードを有効にする
    ///
    /// デバッグモードでは、パース処理の詳細ログが出力されます。
    /// 現在は実装されていませんが、将来のデバッグ機能拡張用です。
    pub fn debug_mode(mut self, enabled: bool) -> Self {
        self.debug_mode = enabled;
        self
    }

    /// 厳密モードを有効にする
    ///
    /// 厳密モードでは、HTMLの構造に厳格な検証を行います。
    /// 現在は実装されていませんが、将来の検証機能拡張用です。
    pub fn strict_mode(mut self, enabled: bool) -> Self {
        self.strict_mode = enabled;
        self
    }

    /// ビルドしてSyllabusSearchResultParserを作成
    pub fn build(self) -> Result<SyllabusSearchResultParser, ParseError> {
        // 現在はビルダーの設定値は使用していませんが、将来の拡張用に保持
        // 将来的にはdebug_modeやstrict_modeをパーサーに渡すことが可能です
        SyllabusSearchResultParser::new()
    }
}
