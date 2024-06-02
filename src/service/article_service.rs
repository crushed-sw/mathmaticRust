use rbatis::plugin::{Page, PageRequest};
use rbatis::Error;
use std::collections::HashSet;
use crate::{
    pool,
    entity::{
        article::{Article, ResponseArticle},
        preview::{Preview, ResponsePreview},
        user::User,
    },
};

pub struct ArticleService;

impl ArticleService {
    pub async fn insert(article: &Article) -> Result<(), Error> {
        let content = match article.content {
            None => "".to_string(),
            Some(ref value) => {
                let arr: Vec<char> = value.chars().collect();
                let slice = &arr[..150.min(arr.len())];
                let res_str: String = slice.iter().collect();
                res_str
            }
        };

        let preview = Preview {
            id: article.id.clone(),
            title: article.title.clone(),
            content: Some(content),
            time: article.time.clone(),
            author: article.author.clone(),
        };

        Article::insert(pool!(), article).await?;
        Preview::insert(pool!(), &preview).await?;

        Ok(())
    }

    pub async fn select_num_preview_page() -> Result<Vec<Preview>, Error> {
        Preview::select_all(pool!()).await
    }

    pub async fn select_preview_page(num: u64) -> Result<Vec<ResponsePreview>, Error> {
        let res = Preview::select_page(pool!(), &PageRequest::new(num * 10 + 1, num * 10 + 10)).await?;
        let mut result_value: Vec<ResponsePreview> = Vec::new();
        let mut userids = Vec::new();
        for item in &res.records {
            userids.push(item.author.unwrap_or_default().to_string());
        }

        let userids: HashSet<String> = userids.into_iter().collect();
        let userids: Vec<&str> = userids.iter().map(|s| s.as_str()).collect();

        let users = User::select_by_method(pool!(), &userids[..]).await?;
        for item in res.records {
            let index = users.iter().position(|user| user.userid == item.author);
            let res = match index {
                None => {
                    ResponsePreview {
                        id: item.id.unwrap_or_default(),
                        title: item.title.unwrap_or_default(),
                        content: item.content.unwrap_or_default(),
                        time: item.time.unwrap_or_default(),
                        userid: "0".to_string(),
                        username: "".to_string(),
                        avatar: "".to_string(),
                    }
                },
                Some(value) => {
                    let user = &users[value];
                    ResponsePreview {
                        id: item.id.unwrap_or_default(),
                        title: item.title.unwrap_or_default(),
                        content: item.content.unwrap_or_default(),
                        time: item.time.unwrap_or_default(),
                        userid: user.userid.clone().unwrap_or_default().to_string(),
                        username: user.username.clone().unwrap_or_default(),
                        avatar: user.avatar.clone().unwrap_or_default(),
                    }
                },
            };
            result_value.push(res);
        }

        Ok(result_value)
    }

    pub async fn select_preview_page_by_author(num: u64, userid: &str) -> Result<Page<Preview>, Error> {
        Preview::select_page_by_userid(pool!(), &PageRequest::new(num * 10 + 1, num * 10 + 10), userid).await
    }

    pub async fn get_preview_num_by_author(userid: &str) -> Result<Vec<Preview>, Error> {
        Preview::select_all_by_userid(pool!(), userid).await
    }

    pub async fn delete_by_id(id: &str) -> Result<(), Error> {
        Preview::delete_by_id(pool!(), id).await?;
        Article::delete_by_id(pool!(), id).await?;

        Ok(())
    }

    pub async fn get_article_by_id(id: &str) -> Result<ResponseArticle, Error> {
        let article_option = Article::select_by_id(pool!(), id).await?;
        match article_option {
            None => Err(Error::from("")),
            Some(article) => {
                let userid = article.author.unwrap().to_string();
                let user_option = User::select_by_id(pool!(), userid.as_str()).await?;
                match user_option {
                    None => Err(Error::from("")),
                    Some(user) => {
                        let res = ResponseArticle {
                            id: article.id.unwrap_or_default(),
                            title: article.title.unwrap_or_default(),
                            content: article.content.unwrap_or_default(),
                            time: article.time.unwrap_or_default(),
                            userid: user.userid.unwrap_or_default(),
                            username: user.username.unwrap_or_default(),
                            avatar: user.avatar.unwrap_or_default(),
                        };
                        Ok(res)
                    }
                }
            }
        }
    }
}
