use super::*;
/// <https://schema.org/issueNumber>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum IssueNumberProperty {
	#[cfg(any(
		any(feature = "integer-schema", feature = "general-schema-section"),
		doc
	))]
	Integer(Integer),
	#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
	Text(Text),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for IssueNumberProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "integer-schema", feature = "general-schema-section"),
					doc
				))]
				IssueNumberProperty::Integer(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
				IssueNumberProperty::Text(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for IssueNumberProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "integer-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Integer as Deserialize>::deserialize(deserializer),
				IssueNumberProperty::Integer,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				IssueNumberProperty::Text,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property issueNumber",
			))
		}
	}
}
