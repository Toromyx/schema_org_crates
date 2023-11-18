use super::*;
/// <https://schema.org/agent>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum AgentProperty {
	/// <https://schema.org/Organization>
	Organization(Organization),
	/// <https://schema.org/Person>
	Person(Person),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for AgentProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				AgentProperty::Organization(ref inner) => inner.serialize(serializer),
				AgentProperty::Person(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				AgentProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for AgentProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			if let Ok(ok) = Result::map(
				<Organization as Deserialize>::deserialize(deserializer),
				AgentProperty::Organization,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Person as Deserialize>::deserialize(deserializer),
				AgentProperty::Person,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				AgentProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property agent or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property agent";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
