use std::collections::HashSet;
use rbatis::Error;
use rbatis::plugin::PageRequest;
use crate::{
    entity::{
        comment::{Comment, ResponseComment},
        user::User,
    },
    pool
};

pub struct CommentService;

impl CommentService {
    pub async fn insert(userid: &str, articleid: &str, content: &str, time: &str) -> Result<(), Error> {
        let comment = Comment {
            id: None,
            userid: Some(userid.parse::<u64>().unwrap_or_default()),
            articleid: Some(articleid.parse::<u64>().unwrap_or_default()),
            content: Some(content.to_string()),
            time: Some(time.to_string()),
        };
        let _ = Comment::insert(pool!(), &comment).await?;
        Ok(())
    }

    pub async fn delete_one(userid: &str, articleid: &str) -> Result<(), Error> {
        let _ = Comment::delete_one(pool!(), userid, articleid).await?;
        Ok(())
    }

    pub async fn delete_by_articleid(articleid: &str) -> Result<(), Error> {
        let _ = Comment::delete_by_article(pool!(), articleid).await?;
        Ok(())
    }

    pub async fn get_num_by_articleid(articleid: &str) -> Result<usize, Error> {
        let comment_vec = Comment::select_all_by_articleid(pool!(), articleid).await?;
        Ok(comment_vec.len())
    }

    pub async fn get_page_by_articleid(articleid: &str, num: u64) -> Result<Vec<ResponseComment>, Error> {
        let comment_vec = Comment::select_page_by_articleid(pool!(), &PageRequest::new(num * 10 + 1, num * 10 + 10), articleid).await?;

        let mut userid_vec: Vec<String> = Vec::new();
        for comment_item in &comment_vec.records {
            userid_vec.push(comment_item.userid.unwrap_or_default().to_string());
        }

        let userid_vec: HashSet<String> = userid_vec.into_iter().collect();
        let userid_vec: Vec<&str> = userid_vec.iter().map(|s| s.as_str()).collect();
        let mut res_vec: Vec<ResponseComment> = Vec::new();

        let users = User::select_by_method(pool!(), &userid_vec[..]).await?;
        for comment_item in comment_vec.records {
            let index = users.iter().position(|user| user.userid == comment_item.userid);
            let comment = match index {
                None => {
                    ResponseComment {
                        userid: comment_item.userid.unwrap(),
                        username: "".to_string(),
                        avatar: "".to_string(),
                        content: comment_item.content.unwrap(),
                        time: comment_item.time.unwrap(),
                    }
                },
                Some(value) => {
                    let user = &users[value];
                    ResponseComment {
                        userid: comment_item.userid.unwrap(),
                        username: user.username.clone().unwrap_or_default(),
                        avatar: user.avatar.clone().unwrap_or_default(),
                        content: comment_item.content.unwrap(),
                        time: comment_item.time.unwrap(),
                    }
                }
            };
            res_vec.push(comment);
        }

        Ok(res_vec)
    }
}
