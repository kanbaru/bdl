use crate::ArrayResult;
use crate::BooruInfo;
use anyhow::anyhow;

pub async fn yandere(info: BooruInfo) -> anyhow::Result<String> {
    let image_result = reqwest::get(&format!(
        "https://{}/post.json?tags=id:{}",
        info.host, info.id
    ))
    .await?
    .json::<ArrayResult>()
    .await
    .unwrap();
    if image_result.is_empty() {
        return Err(anyhow!("Post not found."));
    }
    Ok(image_result[0].file_url.clone())
}

pub async fn yandere_parser(url: url::Url, host: String) -> anyhow::Result<BooruInfo> {
    let pairs = url.path();
    let id = pairs.split("/").collect::<Vec<&str>>()[3]
        .parse::<i64>()
        .unwrap();
    let ret = BooruInfo { host: host, id: id };

    Ok(ret)
}
