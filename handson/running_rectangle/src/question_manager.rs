use crate::constants::{DELTA, INFINITY, INTEGRAL, LAMBDA, MHU, PHI, PI, SIGMA};
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
        question_1.add_answer(Answer::new(0, SIGMA.to_string(), false));
        question_1.add_answer(Answer::new(1, DELTA.to_string(), false));
        question_1.add_answer(Answer::new(2, INTEGRAL.to_string(), false));
        question_1.add_answer(Answer::new(3, INFINITY.to_string(), false));
        question_1.add_answer(Answer::new(4, PI.to_string(), true));

        let mut question_2 = Question::new(2, "15 mod 3 = ?".to_string());
        question_2.add_answer(Answer::new(0, "3".to_string(), false));
        question_2.add_answer(Answer::new(1, "5".to_string(), false));
        question_2.add_answer(Answer::new(2, "0".to_string(), true));
        question_2.add_answer(Answer::new(3, " ".to_string(), false));
        question_2.add_answer(Answer::new(4, SIGMA.to_string(), false));

        let mut question_3 = Question::new(3, "Üçgenin iç açıları toplamıdır...".to_string());
        question_3.add_answer(Answer::new(0, "360°".to_string(), false));
        question_3.add_answer(Answer::new(1, "180°".to_string(), true));
        question_3.add_answer(Answer::new(2, "90°".to_string(), false));
        question_3.add_answer(Answer::new(3, PHI.to_string(), false));
        question_3.add_answer(Answer::new(4, "270°".to_string(), false));

        let mut question_4 = Question::new(4, "0! = ?".to_string());
        question_4.add_answer(Answer::new(0, "1".to_string(), true));
        question_4.add_answer(Answer::new(1, "0".to_string(), false));
        question_4.add_answer(Answer::new(2, DELTA.to_string(), false));
        question_4.add_answer(Answer::new(3, INFINITY.to_string(), false));
        question_4.add_answer(Answer::new(4, "-1".to_string(), false));

        let mut question_5 = Question::new(5, "10 tabanında log(100) = ?".to_string());
        question_5.add_answer(Answer::new(0, LAMBDA.to_string(), false));
        question_5.add_answer(Answer::new(1, "0".to_string(), false));
        question_5.add_answer(Answer::new(2, "2".to_string(), true));
        question_5.add_answer(Answer::new(3, "10".to_string(), false));
        question_5.add_answer(Answer::new(4, DELTA.to_string(), false));

        let mut question_6 = Question::new(6, "Bir zarın kaç yüzeyi vardır?".to_string());
        question_6.add_answer(Answer::new(0, "3".to_string(), false));
        question_6.add_answer(Answer::new(1, MHU.to_string(), false));
        question_6.add_answer(Answer::new(2, "6".to_string(), true));
        question_6.add_answer(Answer::new(3, "12".to_string(), false));
        question_6.add_answer(Answer::new(4, "0".to_string(), false));

        let questions = vec![
            question_1, question_2, question_3, question_4, question_5, question_6,
        ];
        Self { questions }
    }

    pub fn get_question(&self, id: u32) -> Option<&Question> {
        self.questions.iter().find(|q| q.get_id() == id)
    }

    pub fn get_question_count(&self) -> u32 {
        self.questions.len() as u32
    }
}
