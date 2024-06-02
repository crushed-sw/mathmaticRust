use crate::{
    entity::{
        preview::{Preview, ResponseFavoritePreview},
        favorite::Favorite,
    },
    pool
};
use rbatis::Error;
use rbatis::plugin::PageRequest;

pub struct FavoriteService;

impl FavoriteService {
    pub async fn insert(userid: &str, articleid: &str) -> Result<(), Error> {
        let userid = userid.parse::<u64>().unwrap();
        let articleid = articleid.parse::<u64>().unwrap();

        let favorite = Favorite { id: None, userid: Some(userid), articleid: Some(articleid) };
        let _ = Favorite::insert(pool!(), &favorite).await?;
        Ok(())
    }

    pub async fn delete_one(userid: &str, article: &str) -> Result<(), Error> {
        let _ = Favorite::delete_one(pool!(), userid, article).await?;
        Ok(())
    }

    pub async fn select_one(userid: &str, article: &str) -> Result<(), Error> {
        let res_option = Favorite::select_one(pool!(), userid, article).await?;
        match res_option {
            None => Err(Error::from("不存在")),
            Some(_) => Ok(()),
        }
    }

    pub async fn delete_by_articleid(articleid: &str) -> Result<(), Error> {
        let _ = Favorite::delete_by_article(pool!(), articleid).await?;
        Ok(())
    }

    pub async fn select_num_by_userid(userid: &str) -> Result<usize, Error> {
        let favorite_vec = Favorite::select_all_by_userid(pool!(), userid).await?;
        Ok(favorite_vec.len())
    }

    pub async fn select_by_userid(userid: &str, num: u64) -> Result<Vec<ResponseFavoritePreview>, Error> {
        let favorite_vec = Favorite::select_page_by_userid(pool!(), &PageRequest::new(num * 10 + 1, num * 10 + 10), userid).await?;
        let mut articleid_vec: Vec<String> = Vec::new();
        for favorite_item in &favorite_vec.records {
            articleid_vec.push(favorite_item.articleid.unwrap_or_default().to_string());
        }

        let articleid_vec: Vec<&str> = articleid_vec.iter().map(|s| s.as_str()).collect();
        let mut res_vec: Vec<ResponseFavoritePreview> = Vec::new();

        let articles = Preview::select_by_method(pool!(), &articleid_vec[..]).await?;
        for article in articles {
            let preview = ResponseFavoritePreview {
                id: article.id.unwrap_or_default(),
                title: article.title.unwrap_or_default(),
                content: article.content.unwrap_or_default(),
                time: article.time.unwrap_or_default(),
            };
            res_vec.push(preview);
        }

        Ok(res_vec)
    }
}
