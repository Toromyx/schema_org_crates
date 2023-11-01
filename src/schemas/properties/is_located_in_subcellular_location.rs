use super::*;
/// <https://schema.org/isLocatedInSubcellularLocation>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum IsLocatedInSubcellularLocationProperty {
	#[cfg(any(
		any(feature = "defined-term-schema", feature = "pending-schema-section"),
		doc
	))]
	DefinedTerm(DefinedTerm),
	#[cfg(any(
		any(feature = "property-value-schema", feature = "general-schema-section"),
		doc
	))]
	PropertyValue(PropertyValue),
	#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
	Url(Url),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::FailValue),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for IsLocatedInSubcellularLocationProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "defined-term-schema", feature = "pending-schema-section"),
					doc
				))]
				IsLocatedInSubcellularLocationProperty::DefinedTerm(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "property-value-schema", feature = "general-schema-section"),
					doc
				))]
				IsLocatedInSubcellularLocationProperty::PropertyValue(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
				IsLocatedInSubcellularLocationProperty::Url(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for IsLocatedInSubcellularLocationProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "defined-term-schema", feature = "pending-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<DefinedTerm as Deserialize>::deserialize(deserializer),
				IsLocatedInSubcellularLocationProperty::DefinedTerm,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "property-value-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<PropertyValue as Deserialize>::deserialize(deserializer),
				IsLocatedInSubcellularLocationProperty::PropertyValue,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Url as Deserialize>::deserialize(deserializer),
				IsLocatedInSubcellularLocationProperty::Url,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property isLocatedInSubcellularLocation",
			))
		}
	}
}
