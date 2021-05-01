// serialize our hex string (our proto type) into an object id (what mongo uses).
use bson::serde_helpers::serialize_hex_string_as_object_id;

// Include these because the build process will add them to the
// types. We want them so we can get objects in and out of
// MongoDB.
use serde::{Deserialize, Deserializer, Serialize};

/// Deserialize an ObjectId (from mongodb) into a hex string (what our proto uses).
fn deserialize_object_id_as_hex_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let id: bson::oid::ObjectId = Deserialize::deserialize(deserializer)?;
    Ok(id.to_string())
}

tonic::include_proto!("todo");
