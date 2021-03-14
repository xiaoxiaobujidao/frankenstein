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
pub struct InlineObject64 {
    /// Unique identifier for the answered query
    #[serde(rename = "inline_query_id")]
    pub inline_query_id: String,
    /// A JSON-serialized array of results for the inline query
    #[serde(rename = "results")]
    pub results: Vec<crate::models::InlineQueryResult>,
    /// The maximum amount of time in seconds that the result of the inline query may be cached on the server. Defaults to 300.
    #[serde(rename = "cache_time", skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i32>,
    /// Pass *True*, if results may be cached on the server side only for the user that sent the query. By default, results may be returned to any user who sends the same query
    #[serde(rename = "is_personal", skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,
    /// Pass the offset that a client should send in the next query with the same text to receive more results. Pass an empty string if there are no more results or if you don't support pagination. Offset length can't exceed 64 bytes.
    #[serde(rename = "next_offset", skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,
    /// If passed, clients will display a button with specified text that switches the user to a private chat with the bot and sends the bot a start message with the parameter *switch\\_pm\\_parameter*
    #[serde(rename = "switch_pm_text", skip_serializing_if = "Option::is_none")]
    pub switch_pm_text: Option<String>,
    /// [Deep-linking](/bots#deep-linking) parameter for the /start message sent to the bot when user presses the switch button. 1-64 characters, only `A-Z`, `a-z`, `0-9`, `_` and `-` are allowed.    *Example:* An inline bot that sends YouTube videos can ask the user to connect the bot to their YouTube account to adapt search results accordingly. To do this, it displays a 'Connect your YouTube account' button above the results, or even before showing any. The user presses the button, switches to a private chat with the bot and, in doing so, passes a start parameter that instructs the bot to return an oauth link. Once done, the bot can offer a [*switch\\_inline*](https://core.telegram.org/bots/api/#inlinekeyboardmarkup) button so that the user can easily return to the chat where they wanted to use the bot's inline capabilities.
    #[serde(rename = "switch_pm_parameter", skip_serializing_if = "Option::is_none")]
    pub switch_pm_parameter: Option<String>,
}

impl InlineObject64 {
    pub fn new(inline_query_id: String, results: Vec<crate::models::InlineQueryResult>) -> InlineObject64 {
        InlineObject64 {
            inline_query_id,
            results,
            cache_time: None,
            is_personal: None,
            next_offset: None,
            switch_pm_text: None,
            switch_pm_parameter: None,
        }
    }
}


