use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiRequest<T> {
    pub data: T,
}
