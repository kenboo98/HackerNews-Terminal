pub struct Comment {
    pub text: String,
    pub replies: Option<Vec<Comment>>,
}