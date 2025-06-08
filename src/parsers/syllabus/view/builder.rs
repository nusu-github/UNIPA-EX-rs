//! シラバス詳細表示パーサーのビルダー
//!
//! ビルダーパターンを使用してシラバス詳細表示パーサーを構築します。

use super::parser::SyllabusViewParser;
use crate::utils::error::ParseError;

/// シラバス詳細表示パーサービルダー
///
/// ビルダーパターンを使用してSyllabusViewParserを段階的に構築します。
/// 現在は設定オプションは少ないですが、将来の拡張を想定した構造になっています。
#[derive(Default, Debug)]
pub struct SyllabusViewParserBuilder {
    debug_mode: bool,
    strict_mode: bool,
    enable_lesson_plan_parsing: bool,
    enable_active_learning_parsing: bool,
}

impl SyllabusViewParserBuilder {
    /// 新しいビルダーインスタンスを作成
    pub fn new() -> Self {
        Self {
            debug_mode: false,
            strict_mode: false,
            enable_lesson_plan_parsing: true,
            enable_active_learning_parsing: true,
        }
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

    /// 授業計画の解析を有効/無効にする
    ///
    /// 将来的には授業計画の解析を選択的に無効にできるようになります。
    /// 現在は実装されていませんが、パフォーマンス最適化用です。
    pub fn enable_lesson_plan_parsing(mut self, enabled: bool) -> Self {
        self.enable_lesson_plan_parsing = enabled;
        self
    }

    /// アクティブラーニング情報の解析を有効/無効にする
    ///
    /// 将来的にはアクティブラーニング情報の解析を選択的に無効にできるようになります。
    /// 現在は実装されていませんが、パフォーマンス最適化用です。
    pub fn enable_active_learning_parsing(mut self, enabled: bool) -> Self {
        self.enable_active_learning_parsing = enabled;
        self
    }

    /// ビルドしてSyllabusViewParserを作成
    pub fn build(self) -> Result<SyllabusViewParser, ParseError> {
        // 現在はビルダーの設定値は使用していませんが、将来の拡張用に保持
        // 将来的には各種設定をパーサーに渡すことが可能です
        SyllabusViewParser::new()
    }
}
