#[macro_use]
extern crate serde;

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "resultCount")]
    pub result_count: usize,
    pub results: Vec<SearchResult>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResult {
    #[serde(rename = "wrapperType")]
    pub wrapper_type: Option<WrapperType>,
    pub kind: Option<Kind>,
    #[serde(rename = "artistId")]
    pub artist_id: Option<usize>,
    #[serde(rename = "collectionId")]
    pub collection_id: Option<usize>,
    #[serde(rename = "trackId")]
    pub track_id: Option<usize>,
    #[serde(rename = "artistName")]
    pub artist_name: Option<String>,
    #[serde(rename = "collectionName")]
    pub collection_name: Option<String>,
    #[serde(rename = "trackName")]
    pub track_name: Option<String>,
    #[serde(rename = "collectionCensoredName")]
    pub collection_censored_name: Option<String>,
    #[serde(rename = "trackCensoredName")]
    pub track_censored_name: Option<String>,
    #[serde(rename = "artistViewUrl")]
    pub artist_view_url: Option<String>,
    #[serde(rename = "collectionViewUrl")]
    pub collection_view_url: Option<String>,
    #[serde(rename = "feedUrl")]
    pub feed_url: Option<String>,
    #[serde(rename = "trackViewUrl")]
    pub track_view_url: Option<String>,
    #[serde(rename = "artworkUrl30")]
    pub artwork_url30: Option<String>,
    #[serde(rename = "artworkUrl60")]
    pub artwork_url60: Option<String>,
    #[serde(rename = "artworkUrl100")]
    pub artwork_url100: Option<String>,
    #[serde(rename = "collectionPrice")]
    pub collection_price: Option<f64>,
    #[serde(rename = "trackPrice")]
    pub track_price: Option<f64>,
    #[serde(rename = "trackRentalPrice")]
    pub track_rental_price: Option<f64>,
    #[serde(rename = "collectionHdPrice")]
    pub collection_hd_price: Option<f64>,
    #[serde(rename = "trackHdPrice")]
    pub track_hd_price: Option<f64>,
    #[serde(rename = "trackHdRentalPrice")]
    pub track_hd_rental_price: Option<f64>,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    #[serde(rename = "collectionExplicitness")]
    pub collection_explicitness: Option<Explicitness>,
    #[serde(rename = "trackExplicitness")]
    pub track_explicitness: Option<Explicitness>,
    #[serde(rename = "trackCount")]
    pub track_count: Option<usize>,
    pub country: Option<Country>,
    pub currency: Option<Currency>,
    #[serde(rename = "primaryGenreName")]
    pub primary_genre_name: Option<String>,
    #[serde(rename = "contentAdvisoryRating")]
    pub content_advisory_rating: Option<ContentAdvisoryRating>,
    #[serde(rename = "artworkUrl600")]
    pub artwork_url600: Option<String>,
    #[serde(rename = "genreIds")]
    pub genre_ids: Option<Vec<String>>,
    pub genres: Option<Vec<String>>,
}
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Explicitness {
    #[serde(rename = "cleaned")]
    Cleaned,
    #[serde(rename = "explicit")]
    Explicit,
    #[serde(rename = "notExplicit")]
    NotExplicit,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ContentAdvisoryRating {
    Clean,
    Explicit,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Country {
    #[serde(rename = "USA")]
    USA,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "USD")]
    USD,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "podcast")]
    Podcast,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WrapperType {
    #[serde(rename = "track")]
    Track,
}

#[derive(Error, Debug)]
pub enum PodcastSearchError {
    #[error("Failed to fetch information for itunes API {source:?}")]
    FetchError{
        #[from]
        source: reqwest::Error
    },
    #[error("Failed to parse response from itunes API {source:?}")]
    ParseError {
        #[from]
        source: serde_json::Error
    },
    #[error("An unknown error occurred!")]
    Unknown,
}

pub async fn search(terms: &str) -> Result<SearchResponse, PodcastSearchError> {
    let encoded: String = utf8_percent_encode(terms, NON_ALPHANUMERIC).to_string();
    let url =  format!("https://itunes.apple.com/search?media=podcast&entity=podcast&term={}", encoded);
    let resp = reqwest::get(&url).await?.json::<SearchResponse>().await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
