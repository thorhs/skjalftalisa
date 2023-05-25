mod request;
mod response;

use reqwest::Client;

pub async fn get_quakes(
    client: &Client,
    req: &request::SkjalftalisaRequest,
) -> Result<response::SkjalftalisaResponse, ()> {
    let resp = client
        .post("https://skjalftalisa-api.vedur.is/v1/quake/array")
        .header("content-type", " application/json")
        .json(req)
        .send()
        .await
        .unwrap();
    let text_resp = resp.text().await.unwrap();

    let resp = if let Ok(text_resp) = serde_json::from_str(&text_resp) {
        text_resp
    } else {
        println!("{}", text_resp);
        unimplemented!();
    };

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo::{LineString, Polygon};
    use chrono::prelude::*;
    use chrono::{Duration, DurationRound};

    #[tokio::test]
    async fn it_works() {
        let client = reqwest::Client::new();
        let request = crate::request::SkjalftalisaRequest::new(Polygon::new(
            LineString::from(vec![
                (67.14815809664168, -18.589553916826844),
                (66.08886951976677, -20.204544151201848),
                (65.68945088379868, -18.084182823076848),
                (66.23539315614204, -16.040725791826848),
            ]),
            vec![],
        ))
        .with_time(
                Utc::now().duration_trunc(Duration::days(1)).unwrap(),
                Utc::now(),
                );
        let result = get_quakes(&client, &request).await.unwrap();
        assert!(result.data.get(0).is_some());
    }
}
