//! 成績照会モジュール
//!
//! UNIVERSAL PASSPORT EXの成績照会ページのパーサー機能を提供します。
//! 三層アーキテクチャ（Builder → Parser → Model）を採用しています。

pub mod builder;
pub mod model;
pub mod parser;

pub use builder::GradeInquiryParserBuilder;
pub use model::{
    CategoryCredits, CreditDetails, CreditSummary, DisplayPattern, DisplaySettings, Grade,
    GradeInquiry, RequirementType, Semester, Subject, SubjectCategory, ViewType,
};
pub use parser::GradeInquiryParserImpl;

/// 既存のパーサー（後方互換性のため）
pub type GradeInquiryParser = GradeInquiryParserBuilder;
