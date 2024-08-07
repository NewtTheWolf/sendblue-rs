use phonenumber::{parse, Mode, PhoneNumber as RawPhoneNumber};
use schemars::{gen::SchemaGenerator, schema::{Schema, SchemaObject}, JsonSchema};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use serde_with::{serde_as, skip_serializing_none, NoneAsEmptyString};
use chrono::{DateTime, Utc};
use validator::Validate;

// Newtype um PhoneNumber
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PhoneNumber(RawPhoneNumber);

impl Deref for PhoneNumber {
    type Target = RawPhoneNumber;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Implement JsonSchema for PhoneNumber
impl JsonSchema for PhoneNumber {
    fn schema_name() -> String {
        "PhoneNumber".to_string()
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
        let schema_object = SchemaObject {
            instance_type: Some(schemars::schema::InstanceType::String.into()),
            format: Some("phone".to_string()),
            ..Default::default()
        };
        Schema::Object(schema_object)
    }
}

impl From<RawPhoneNumber> for PhoneNumber {
    fn from(phone_number: RawPhoneNumber) -> Self {
        PhoneNumber(phone_number)
    }
}


/* pub fn deserialize_phone_number<'de, D>(deserializer: D) -> Result<PhoneNumber, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    parse(None, s).map_err(serde::de::Error::custom)
} */

/* pub fn deserialize_option_phone_number<'de, D>(
    deserializer: D,
) -> Result<Option<PhoneNumber>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<&str> = Option::deserialize(deserializer)?;
    match opt {
        Some(s) => parse(None, s).map(Some).map_err(serde::de::Error::custom),
        None => Ok(None),
    }
} */

/* pub fn deserialize_vec_phone_number<'de, D>(deserializer: D) -> Result<Vec<PhoneNumber>, D::Error>
where
    D: Deserializer<'de>,
{
    let vec: Vec<&str> = Deserialize::deserialize(deserializer)?;
    vec.into_iter()
        .map(|s| parse(None, s).map_err(serde::de::Error::custom))
        .collect()
} */

/* pub fn deserialize_option_vec_phone_number<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<PhoneNumber>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<Vec<&str>> = Option::deserialize(deserializer)?;
    match opt {
        Some(vec) => {
            let mut phone_numbers = Vec::new();
            for s in vec {
                match parse(None, s) {
                    Ok(phone_number) => phone_numbers.push(phone_number),
                    Err(err) => return Err(serde::de::Error::custom(err)),
                }
            }
            Ok(Some(phone_numbers))
        }
        None => Ok(None),
    }
} */

pub fn serialize_phone_number<S>(number: &PhoneNumber, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let e164 = number.format().mode(Mode::E164).to_string();
    serializer.serialize_str(&e164)
}

