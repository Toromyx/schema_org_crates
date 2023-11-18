use super::*;
/// <https://schema.org/requiredQuantity>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum RequiredQuantityProperty {
	/// <https://schema.org/QuantitativeValue>
	QuantitativeValue(QuantitativeValue),
	/// <https://schema.org/Number>
	Number(Number),
	/// <https://schema.org/Text>
	Text(Text),
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
	impl Serialize for RequiredQuantityProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				RequiredQuantityProperty::QuantitativeValue(ref inner) => {
					inner.serialize(serializer)
				}
				RequiredQuantityProperty::Number(ref inner) => inner.serialize(serializer),
				RequiredQuantityProperty::Text(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				RequiredQuantityProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for RequiredQuantityProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			if let Ok(ok) = Result::map(
				<QuantitativeValue as Deserialize>::deserialize(deserializer),
				RequiredQuantityProperty::QuantitativeValue,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Number as Deserialize>::deserialize(deserializer),
				RequiredQuantityProperty::Number,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				RequiredQuantityProperty::Text,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				RequiredQuantityProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property requiredQuantity or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property requiredQuantity";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
