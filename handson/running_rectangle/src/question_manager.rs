use crate::entity::{Answer, Question};

pub struct QuestionManager {
    questions: Vec<Question>,
}

impl QuestionManager {
    pub fn init() -> Self {
        let mut question_1 = Question::new(
            1,
            "3.14 olarak da bilinen Matematik enstrümanıdır?".to_string(),
        );
        question_1.add_answer(Answer::new("".to_string(), false));
        question_1.add_answer(Answer::new("P".to_string(), false));
        question_1.add_answer(Answer::new("e".to_string(), false));
        question_1.add_answer(Answer::new("N".to_string(), false));
        question_1.add_answer(Answer::new("\u{03C0}".to_string(), true));

        let questions = vec![question_1];
        Self { questions }
    }

    pub fn get_question(&self, id: u32) -> Option<&Question> {
        self.questions.iter().find(|q| q.get_id() == id)
    }
}
