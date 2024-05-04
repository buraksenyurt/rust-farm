use crate::constants::{DELTA, INFINITY, INTEGRAL, LAMBDA, MHU, PHI, PI, SIGMA};
use crate::entity::{Answer, Level, Question};

pub struct QuestionManager {
    questions: Vec<Question>,
}

impl QuestionManager {
    pub fn init() -> Self {
        let mut question_1 = Question::new(
            1,
            "3.14 olarak da bilinen Matematik enstrümanıdır?".to_string(),
            Level::Easy,
        );
        question_1.add_answer(Answer::new(0, SIGMA.to_string(), false));
        question_1.add_answer(Answer::new(1, DELTA.to_string(), false));
        question_1.add_answer(Answer::new(2, INTEGRAL.to_string(), false));
        question_1.add_answer(Answer::new(3, INFINITY.to_string(), false));
        question_1.add_answer(Answer::new(4, PI.to_string(), true));

        let mut question_2 = Question::new(2, "15 mod 3 = ?".to_string(), Level::Easy);
        question_2.add_answer(Answer::new(0, "3".to_string(), false));
        question_2.add_answer(Answer::new(1, "5".to_string(), false));
        question_2.add_answer(Answer::new(2, "0".to_string(), true));
        question_2.add_answer(Answer::new(3, " ".to_string(), false));
        question_2.add_answer(Answer::new(4, SIGMA.to_string(), false));

        let mut question_3 = Question::new(
            3,
            "Üçgenin iç açıları toplamıdır...".to_string(),
            Level::Easy,
        );
        question_3.add_answer(Answer::new(0, "360°".to_string(), false));
        question_3.add_answer(Answer::new(1, "180°".to_string(), true));
        question_3.add_answer(Answer::new(2, "90°".to_string(), false));
        question_3.add_answer(Answer::new(3, PHI.to_string(), false));
        question_3.add_answer(Answer::new(4, "270°".to_string(), false));

        let mut question_4 = Question::new(4, "0! = ?".to_string(), Level::Easy);
        question_4.add_answer(Answer::new(0, "1".to_string(), true));
        question_4.add_answer(Answer::new(1, "0".to_string(), false));
        question_4.add_answer(Answer::new(2, DELTA.to_string(), false));
        question_4.add_answer(Answer::new(3, INFINITY.to_string(), false));
        question_4.add_answer(Answer::new(4, "-1".to_string(), false));

        let mut question_5 = Question::new(5, "10 tabanında log(100) = ?".to_string(), Level::Easy);
        question_5.add_answer(Answer::new(0, LAMBDA.to_string(), false));
        question_5.add_answer(Answer::new(1, "0".to_string(), false));
        question_5.add_answer(Answer::new(2, "2".to_string(), true));
        question_5.add_answer(Answer::new(3, "10".to_string(), false));
        question_5.add_answer(Answer::new(4, DELTA.to_string(), false));

        let mut question_6 =
            Question::new(6, "Bir zarın kaç yüzeyi vardır?".to_string(), Level::Easy);
        question_6.add_answer(Answer::new(0, "3".to_string(), false));
        question_6.add_answer(Answer::new(1, MHU.to_string(), false));
        question_6.add_answer(Answer::new(2, "6".to_string(), true));
        question_6.add_answer(Answer::new(3, "12".to_string(), false));
        question_6.add_answer(Answer::new(4, "0".to_string(), false));

        let mut question_7 = Question::new(7, "49un karekökü kaçtır?".to_string(), Level::Easy);
        question_7.add_answer(Answer::new(0, "7".to_string(), true));
        question_7.add_answer(Answer::new(1, PI.to_string(), false));
        question_7.add_answer(Answer::new(2, INFINITY.to_string(), false));
        question_7.add_answer(Answer::new(3, "-7".to_string(), false));
        question_7.add_answer(Answer::new(4, "?".to_string(), false));

        let mut question_8 = Question::new(
            8,
            "3x − 7 = 2x + 8 denkleminde x değeri nedir?".to_string(),
            Level::Hard,
        );
        question_8.add_answer(Answer::new(0, "15".to_string(), true));
        question_8.add_answer(Answer::new(1, "19".to_string(), false));
        question_8.add_answer(Answer::new(2, INFINITY.to_string(), false));
        question_8.add_answer(Answer::new(3, "1".to_string(), false));
        question_8.add_answer(Answer::new(4, DELTA.to_string(), false));

        let mut question_9 = Question::new(
            9,
            "Fiyatı 15 TL olan bir ürün üzerine %20 KDV eklenirse KDV dahil fiyatı ne olur?"
                .to_string(),
            Level::Medium,
        );
        question_9.add_answer(Answer::new(0, "20".to_string(), false));
        question_9.add_answer(Answer::new(1, "15".to_string(), false));
        question_9.add_answer(Answer::new(2, "1".to_string(), false));
        question_9.add_answer(Answer::new(3, "18".to_string(), true));
        question_9.add_answer(Answer::new(4, SIGMA.to_string(), false));

        let questions = vec![
            question_1, question_2, question_3, question_4, question_5, question_6, question_7,
            question_8, question_9,
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
