//! テスト解答状況パーサーのビルダー
//!
//! パーサーの設定を管理し、TestAnswerStatusParserImplインスタンスを生成する
//! ビルダーパターンの実装を提供します。

use super::parser::TestAnswerStatusParserImpl;
use crate::common::traits::DataBuilder;
use crate::utils::error::ParseError;

/// テスト解答状況パーサーのビルダー
#[derive(Debug, Default)]
pub struct TestAnswerStatusParserBuilder {
    /// デバッグモードの有効/無効
    debug_mode: bool,
    /// 厳密なパースモード（エラー時に停止するかどうか）
    strict_mode: bool,
}

impl TestAnswerStatusParserBuilder {
    /// 新しいビルダーインスタンスを作成
    pub fn new() -> Self {
        Self::default()
    }

    /// デバッグモードを有効にする
    pub fn with_debug_mode(mut self, enabled: bool) -> Self {
        self.debug_mode = enabled;
        self
    }

    /// 厳密なパースモードを設定する
    pub fn with_strict_mode(mut self, enabled: bool) -> Self {
        self.strict_mode = enabled;
        self
    }

    /// パーサーインスタンスを構築する
    pub fn build(self) -> Result<TestAnswerStatusParserImpl, ParseError> {
        TestAnswerStatusParserImpl::new_with_config(self.debug_mode, self.strict_mode)
    }
}

impl DataBuilder<TestAnswerStatusParserImpl> for TestAnswerStatusParserBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn build(self) -> Result<TestAnswerStatusParserImpl, ParseError> {
        self.build()
    }

    fn validate(&self, _parser: &TestAnswerStatusParserImpl) -> Result<(), ParseError> {
        // パーサーの設定に問題がないかを検証
        // 現在は特に検証項目なし
        Ok(())
    }
}
