use crate::BooruInfo;
use serde;
use serde::{Deserialize, Serialize};

type GelbooruResults = Vec<Image>;

#[derive(Debug, Serialize, Deserialize)]
struct Image {
    id: i64,
    file_url: String,
}

pub async fn gelbooru(info: BooruInfo) -> anyhow::Result<String> {
    let image_result =
        reqwest::get(&format!("https://{}/index.php?page=dapi&s=post&q=index&id={}&json=1", info.host, info.id))
            .await?
            .json::<GelbooruResults>()
            .await?;
    Ok(image_result[0].file_url.clone())
}

pub async fn gelbooru_parser(url: url::Url) -> anyhow::Result<BooruInfo> {
    let mut pairs = url.query_pairs();
    let mut id = 0;
    while id == 0 {
        let result = pairs.next();
        let unwrap = result.unwrap();
        if unwrap.clone().0.to_mut() == "id" {
            id = unwrap.clone().1.to_mut().parse::<i64>().unwrap()
        }
    }
    let ret = BooruInfo {
        host: url.host_str().unwrap().to_string(),
        id: id,
    };

    Ok(ret)
}