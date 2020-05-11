use crate::hn_api::get_comments;
use ammonia::Builder;
use std::collections::HashSet;

pub struct Comment {
    pub text: String,
    pub replies: Option<Vec<Comment>>,
}

pub struct CommentBlock {
    pub comments: Vec<Comment>,
    pub comment_strings: Vec<String>
}

impl CommentBlock {
    fn helper(c: &Comment, depth: u16, builder: &Builder) -> Vec<String> {
        let prefix = "-".repeat(depth as usize);
        let mut result = vec![format!("{}{}\n", prefix, builder.clean(c.text.as_str()))];
        match &c.replies {
            Some(replies) => {
                for reply in replies {
                    result.append(&mut CommentBlock::helper(reply, depth + 1, builder));
                }
            },
            None => {}
        }
        return result
    }
    pub fn new(comment_ids: &[i64]) -> CommentBlock {
        let comments = match get_comments(comment_ids) {
            Ok(c) => c,
            Err(_) => {
                return CommentBlock {
                   comments: Vec::new(),
                   comment_strings: Vec::new()
               };
            }
        };
        let mut builder = Builder::new();
        let tag_cleaner = builder.tags(HashSet::new());
        let mut comment_strings = Vec::new();
        for c in &comments {
            comment_strings.append(&mut CommentBlock::helper(c, 1, &tag_cleaner));
        };

        CommentBlock {
            comments,
            comment_strings
        }

    }

}