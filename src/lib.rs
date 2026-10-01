use trpl::get;
use trpl::Html;

pub async fn get_page_title(url: &String) -> Result<(&String, Option<String>), String> {
    let data = get(url).await.text().await;
    let html = Html::parse(&data);

    let first_title = html.select_first("title");
    match first_title {
        None => return Ok((url, None)),
        Some(title) => return Ok((url, Some(title.html())))
    }
}
