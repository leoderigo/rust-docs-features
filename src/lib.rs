use trpl::get;
use trpl::Html;

pub async fn get_page_title(url: String) -> (String, Option<String>) {
    let data = get(&url).await.text().await;
    let title = Html::parse(&data)
        .select_first("title")
        .map(|title| title.inner_html());

    (url, title)
}
