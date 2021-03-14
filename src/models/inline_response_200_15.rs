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
pub struct InlineResponse20015 {
    #[serde(rename = "ok")]
    pub ok: bool,
    #[serde(rename = "result")]
    pub result: crate::models::ChatMember,
}

impl InlineResponse20015 {
    pub fn new(ok: bool, result: crate::models::ChatMember) -> InlineResponse20015 {
        InlineResponse20015 {
            ok,
            result,
        }
    }
}


