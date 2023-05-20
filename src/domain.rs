use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    code: String,
    message: String,
    data: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    id: String,
    tag: String,
    name: String,
    message: String,
    #[serde(rename = "createTime")]
    create_time: String,
    #[serde(rename = "modifyTime")]
    modify_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscriber {
    id: String,
    tag: String,
    email: String,
}

#[cfg(test)]
mod tests {
    use crate::domain::{Event, Response};

    #[test]
    fn from_str() {
        let json = r#"{"code":"200","message":"success","data":{"id":"7","tag":"error","name":"event","message":"test","createTime":"2023-05-20T18:22:21.421063","modifyTime":"2023-05-20T18:22:21.421081"}}"#;
        let response = serde_json::from_str::<Response<Event>>(json).unwrap();
        println!("{:#?}", response);
    }
}
