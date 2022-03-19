
use std::collections::HashSet;

struct ValidQuestionnaireResponse {
    pub id: i32,
    pub answer: Option<HashSet<i32>>,
    pub answer_str: Option<String>
}
