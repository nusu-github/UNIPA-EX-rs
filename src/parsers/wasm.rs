// Export all parsers for use in lib.rs
pub use crate::parsers::assignment_submit::AssignmentSubmitParser;
pub use crate::parsers::classroom_reservation_status::ClassroomReservationStatusParser;
pub use crate::parsers::grades::GradesParser;
pub use crate::parsers::portal::{PortalParser, PortalAllNotificationsParser, PortalClassContactParser, PortalAllClassContactParser};
pub use crate::parsers::portal::notification_detail::NotificationDetailParser;
pub use crate::parsers::questionnaire::QuestionnaireParser;
pub use crate::parsers::student_info_inquiry::StudentInfoInquiryParser;
pub use crate::parsers::syllabus::SyllabusParser;
pub use crate::parsers::test_answer_status::TestAnswerStatusParser;
