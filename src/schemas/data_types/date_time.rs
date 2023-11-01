use super::*;
/// <https://schema.org/DateTime>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct DateTime(pub crate::date_types::DateTime);
impl std::ops::Deref for DateTime {
	type Target = crate::date_types::DateTime;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for DateTime {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			serializer.serialize_newtype_struct("DateTime", {
				struct SerializeWith<'a>(&'a crate::date_types::DateTime);
				impl<'a> Serialize for SerializeWith<'a> {
					fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
					where
						S: Serializer,
					{
						serde_with::As::<serde_with::DisplayFromStr>::serialize(self.0, serializer)
					}
				}
				&SerializeWith(&self.0)
			})
		}
	}
	impl<'de> Deserialize<'de> for DateTime {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			struct DataTypeVisitor;
			impl<'de> Visitor<'de> for DataTypeVisitor {
				type Value = DateTime;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema DateTime")
				}
				fn visit_newtype_struct<E>(self, e: E) -> Result<Self::Value, E::Error>
				where
					E: Deserializer<'de>,
				{
					let inner: crate::date_types::DateTime =
						serde_with::As::<serde_with::DisplayFromStr>::deserialize(e)?;
					Ok(DateTime(inner))
				}
			}
			Deserializer::deserialize_newtype_struct(deserializer, "DateTime", DataTypeVisitor)
		}
	}
}
