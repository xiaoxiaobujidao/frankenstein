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
pub struct InlineObject51 {
    /// Required if *inline\\_message\\_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[serde(rename = "chat_id", skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<crate::models::AnyOfintegerstring>,
    /// Required if *inline\\_message\\_id* is not specified. Identifier of the message to edit
    #[serde(rename = "message_id", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    /// Required if *chat\\_id* and *message\\_id* are not specified. Identifier of the inline message
    #[serde(rename = "inline_message_id", skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// New caption of the message, 0-1024 characters after entities parsing
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the message caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(rename = "parse_mode", skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of *parse\\_mode*
    #[serde(rename = "caption_entities", skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::models::MessageEntity>>,
    #[serde(rename = "reply_markup", skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::models::InlineKeyboardMarkup>,
}

impl InlineObject51 {
    pub fn new() -> InlineObject51 {
        InlineObject51 {
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
        }
    }
}


