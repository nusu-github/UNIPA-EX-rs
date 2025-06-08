//! シラバス検索フォームのビルダー
//!
//! ビルダーパターンを使用してシラバス検索フォームを構築します。

use super::model::SyllabusSearchForm;
use crate::utils::error::ParseError;

/// シラバス検索フォームビルダー
///
/// ビルダーパターンを使用してSyllabusSearchFormを段階的に構築します。
/// メソッドチェーンによって設定値を指定できます。
#[derive(Default, Debug)]
pub struct SyllabusSearchFormBuilder {
    form: SyllabusSearchForm,
}

impl SyllabusSearchFormBuilder {
    /// 新しいビルダーインスタンスを作成
    pub fn new() -> Self {
        Self::default()
    }

    /// 管理部署名を設定
    pub fn kanri_bsyo_name<S: Into<String>>(mut self, value: S) -> Self {
        self.form.kanri_bsyo_name = Some(value.into());
        self
    }

    /// 年度を設定
    pub fn nendo(mut self, value: u32) -> Self {
        self.form.nendo = Some(value);
        self
    }

    /// 学期を設定
    pub fn gakki_no<S: Into<String>>(mut self, value: S) -> Self {
        self.form.gakki_no = Some(value.into());
        self
    }

    /// 科目区分を設定
    pub fn kamok_jugyo<S: Into<String>>(mut self, value: S) -> Self {
        self.form.kamok_jugyo = Some(value.into());
        self
    }

    /// 科目名を設定
    pub fn kamok_name<S: Into<String>>(mut self, value: S) -> Self {
        self.form.kamok_name = Some(value.into());
        self
    }

    /// 担当教員を設定
    pub fn kyoin_simei<S: Into<String>>(mut self, value: S) -> Self {
        self.form.kyoin_simei = Some(value.into());
        self
    }

    /// 学科・コース／専攻を設定
    pub fn gakka<S: Into<String>>(mut self, value: S) -> Self {
        self.form.gakka = Some(value.into());
        self
    }

    /// 学年を設定
    pub fn gakunen<S: Into<String>>(mut self, value: S) -> Self {
        self.form.gakunen = Some(value.into());
        self
    }

    /// 曜日を設定
    pub fn yobi<S: Into<String>>(mut self, value: S) -> Self {
        self.form.yobi = Some(value.into());
        self
    }

    /// 時限を設定
    pub fn jigen<S: Into<String>>(mut self, value: S) -> Self {
        self.form.jigen = Some(value.into());
        self
    }

    /// 集中講義フラグを設定
    pub fn syutyu(mut self, value: bool) -> Self {
        self.form.syutyu = Some(value);
        self
    }

    /// キーワードを設定
    pub fn keyword<S: Into<String>>(mut self, value: S) -> Self {
        self.form.keyword = Some(value.into());
        self
    }

    /// 識別区分を設定
    pub fn shikibetsu_kbn(mut self, value: u32) -> Self {
        self.form.shikibetsu_kbn = Some(value);
        self
    }

    /// 管理番号を設定
    pub fn kanri_no(mut self, value: u32) -> Self {
        self.form.kanri_no = Some(value);
        self
    }

    /// ビルドしてSyllabusSearchFormを作成
    pub fn build(self) -> Result<SyllabusSearchForm, ParseError> {
        Ok(self.form)
    }
}
