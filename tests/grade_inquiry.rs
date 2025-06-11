use unipa_ex::parsers::grades::grade_inquiry::{GradeInquiryParserBuilder, Grade};
use scraper::Html;

fn sample_html_with_gpa() -> &'static str {
    r#"
    <div class="gpaScore">GPA: 3.50</div>
    <table class="listTable">
      <tbody>
        <tr>
          <td>Programming I</td>
          <td>2</td>
          <td>秀</td>
          <td>Prof. A</td>
          <td>-</td>
        </tr>
      </tbody>
    </table>
    <table class="creditSummary">
      <tr><td>全体</td><td>124</td><td>2</td><td>0</td></tr>
    </table>
    "#
}

fn sample_html_without_gpa() -> &'static str {
    r#"
    <table class="listTable">
      <tbody>
        <tr>
          <td>Programming I</td>
          <td>2</td>
          <td>優</td>
          <td>Prof. A</td>
          <td>-</td>
        </tr>
      </tbody>
    </table>
    "#
}

#[test]
fn parse_basic_grade_inquiry() {
    let html = Html::parse_document(sample_html_with_gpa());
    let parser = GradeInquiryParserBuilder::new().build().unwrap();
    let result = parser.parse_document(&html).unwrap();

    assert_eq!(result.subjects.len(), 1);
    assert_eq!(result.subjects[0].name, "Programming I");
    assert!(matches!(result.subjects[0].grade, Some(Grade::AA)));
    assert!((result.gpa_score - 3.5).abs() < f64::EPSILON);
    assert_eq!(result.credit_summary.overall.total_credits, 2);
}

#[test]
fn parse_without_gpa_calculates_from_grades() {
    let html = Html::parse_document(sample_html_without_gpa());
    let parser = GradeInquiryParserBuilder::new().build().unwrap();
    let result = parser.parse_document(&html).unwrap();

    assert_eq!(result.gpa_score, 3.0);
    assert_eq!(result.credit_summary.overall.total_credits, 2);
}
