use chariott_common::{
    chariott_api::ChariottCommunication,
    error::{Error, ResultExt as _},
};
use chariott_proto::{runtime::{FulfillRequest, FulfillResponse}, common::IntentEnum};
use prost::Message;

// Fulfills a message against Chariott.
pub async fn fulfill(
    chariott: &mut impl ChariottCommunication,
    message: &[u8],
) -> Result<FulfillResponse, Error> {
    let fulfill_request: FulfillRequest =
        Message::decode(message).map_err_with("Could not decode message.")?;

    // Fulfill all requests against Chariott, without distinguishing between
    // subscription- and non-subscription related requests.

    let intent_enum = fulfill_request
        .intent
        .and_then(|intent| intent.intent)
        .ok_or_else(|| Error::new("Message does not contain an intent."))?;

    match intent_enum {
        IntentEnum::Discover(_) => Err(Error::new("Discover is not supported.")),
        IntentEnum::Subscribe(_) => todo!(),
        _ => chariott.fulfill(fulfill_request.namespace, intent_enum).await.map(|r| r.into_inner()),
    }
}
