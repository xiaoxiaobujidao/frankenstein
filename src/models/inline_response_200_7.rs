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
pub struct InlineResponse2007 {
    #[serde(rename = "ok")]
    pub ok: bool,
    #[serde(rename = "result")]
    pub result: crate::models::AnyOfMessageboolean,
}

impl InlineResponse2007 {
    pub fn new(ok: bool, result: crate::models::AnyOfMessageboolean) -> InlineResponse2007 {
        InlineResponse2007 {
            ok,
            result,
        }
    }
}


