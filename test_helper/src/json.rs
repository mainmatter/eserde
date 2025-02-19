use arbitrary::Arbitrary;
use fake::{Fake, Faker};


/// Wrapper around [`serde_json::Value`] that implements the traits needed
/// for use in tests and fuzz targets.
#[derive(crate::prelude::Deserialize, crate::prelude::Serialize, std::fmt::Debug, fake::Dummy)]
pub struct JsonValue(#[eserde(compat)] serde_json::Value);

impl<'a> arbitrary::Arbitrary<'a> for JsonValue {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let value = match u8::arbitrary(u)? % 5 {
            0 => serde_json::Value::Null,
            1 => serde_json::Value::Bool(Arbitrary::arbitrary(u)?),
            2 => serde_json::Value::Number(JsonNumber::arbitrary(u)?.0),
            3 => serde_json::Value::String(Arbitrary::arbitrary(u)?),
            4 => {
                let items = u.arbitrary_iter::<NonRecursiveValue>()?.try_fold(
                    Vec::new(),
                    |mut items, v| {
                        items.push(v?.into());
                        Ok(items)
                    },
                )?;
                serde_json::Value::Array(items)
            }
            5.. => unreachable!("We %'d by 5 just now"),
        };

        Ok(Self(value))
    }
}

impl JsonValue {
    /// Convenience method that calls [`Faker::fake`].
    /// Saves us from depending on [`fake`] in other crates or re-exporting it.
    pub fn fake() -> Self {
        Faker.fake()
    }
}

#[derive(Arbitrary)]
/// JSON value that doesn't hold an array or map,
/// so as to not blow up the stack when generating arbitrary values.
/// Based on `<serde_json::Value as fake::Dummy<Faker>>::dummy_with_rng`
enum NonRecursiveValue {
    Null,
    Bool(bool),
    Number(JsonNumber),
    String(String),
}

impl From<NonRecursiveValue> for serde_json::Value {
    fn from(value: NonRecursiveValue) -> Self {
        match value {
            NonRecursiveValue::Null => Self::Null,
            NonRecursiveValue::Bool(b) => Self::Bool(b),
            NonRecursiveValue::Number(json_number) => Self::Number(json_number.0),
            NonRecursiveValue::String(s) => Self::String(s),
        }
    }
}

pub struct JsonNumber(pub serde_json::Number);

impl<'a> Arbitrary<'a> for JsonNumber {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let number = match bool::arbitrary(u)? {
            true => serde_json::Number::from_f64(Arbitrary::arbitrary(u)?),
            false => serde_json::Number::from_i128(Arbitrary::arbitrary(u)?),
        };
        Ok(Self(number.ok_or(arbitrary::Error::IncorrectFormat)?))
    }
}
