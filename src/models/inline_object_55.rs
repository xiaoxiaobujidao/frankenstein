/*
 * Telegram Bot API
 *
 * Auto-generated OpenAPI schema
 *
 * The version of the OpenAPI document: 5.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineObject55 {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[serde(rename = "chat_id")]
    pub chat_id: crate::models::AnyOfintegerstring,
    /// Identifier of the message to delete
    #[serde(rename = "message_id")]
    pub message_id: i32,
}

impl InlineObject55 {
    pub fn new(chat_id: crate::models::AnyOfintegerstring, message_id: i32) -> InlineObject55 {
        InlineObject55 {
            chat_id,
            message_id,
        }
    }
}


