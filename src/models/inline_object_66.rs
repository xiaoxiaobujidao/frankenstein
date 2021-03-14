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
pub struct InlineObject66 {
    /// Unique identifier for the query to be answered
    #[serde(rename = "shipping_query_id")]
    pub shipping_query_id: String,
    /// Specify True if delivery to the specified address is possible and False if there are any problems (for example, if delivery to the specified address is not possible)
    #[serde(rename = "ok")]
    pub ok: bool,
    /// Required if *ok* is True. A JSON-serialized array of available shipping options.
    #[serde(rename = "shipping_options", skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<crate::models::ShippingOption>>,
    /// Required if *ok* is False. Error message in human readable form that explains why it is impossible to complete the order (e.g. \"Sorry, delivery to your desired address is unavailable'). Telegram will display this message to the user.
    #[serde(rename = "error_message", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl InlineObject66 {
    pub fn new(shipping_query_id: String, ok: bool) -> InlineObject66 {
        InlineObject66 {
            shipping_query_id,
            ok,
            shipping_options: None,
            error_message: None,
        }
    }
}


