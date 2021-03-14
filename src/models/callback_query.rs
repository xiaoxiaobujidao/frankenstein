/*
 * Telegram Bot API
 *
 * Auto-generated OpenAPI schema
 *
 * The version of the OpenAPI document: 5.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CallbackQuery : This object represents an incoming callback query from a callback button in an [inline keyboard](/bots#inline-keyboards-and-on-the-fly-updating). If the button that originated the query was attached to a message sent by the bot, the field *message* will be present. If the button was attached to a message sent via the bot (in [inline mode](https://core.telegram.org/bots/api/#inline-mode)), the field *inline\\_message\\_id* will be present. Exactly one of the fields *data* or *game\\_short\\_name* will be present.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CallbackQuery {
    /// Unique identifier for this query
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "from")]
    pub from: crate::models::User,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<crate::models::Message>,
    /// *Optional*. Identifier of the message sent via the bot in inline mode, that originated the query.
    #[serde(rename = "inline_message_id", skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Global identifier, uniquely corresponding to the chat to which the message with the callback button was sent. Useful for high scores in [games](https://core.telegram.org/bots/api/#games).
    #[serde(rename = "chat_instance")]
    pub chat_instance: String,
    /// *Optional*. Data associated with the callback button. Be aware that a bad client can send arbitrary data in this field.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// *Optional*. Short name of a [Game](https://core.telegram.org/bots/api/#games) to be returned, serves as the unique identifier for the game
    #[serde(rename = "game_short_name", skip_serializing_if = "Option::is_none")]
    pub game_short_name: Option<String>,
}

impl CallbackQuery {
    /// This object represents an incoming callback query from a callback button in an [inline keyboard](/bots#inline-keyboards-and-on-the-fly-updating). If the button that originated the query was attached to a message sent by the bot, the field *message* will be present. If the button was attached to a message sent via the bot (in [inline mode](https://core.telegram.org/bots/api/#inline-mode)), the field *inline\\_message\\_id* will be present. Exactly one of the fields *data* or *game\\_short\\_name* will be present.
    pub fn new(id: String, from: crate::models::User, chat_instance: String) -> CallbackQuery {
        CallbackQuery {
            id,
            from,
            message: None,
            inline_message_id: None,
            chat_instance,
            data: None,
            game_short_name: None,
        }
    }
}


