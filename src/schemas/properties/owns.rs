use super::*;
/// <https://schema.org/owns>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum OwnsProperty {
	#[cfg(any(
		any(feature = "ownership-info-schema", feature = "general-schema-section"),
		doc
	))]
	OwnershipInfo(OwnershipInfo),
	#[cfg(any(
		any(feature = "product-schema", feature = "general-schema-section"),
		doc
	))]
	Product(Product),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for OwnsProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "ownership-info-schema", feature = "general-schema-section"),
					doc
				))]
				OwnsProperty::OwnershipInfo(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "product-schema", feature = "general-schema-section"),
					doc
				))]
				OwnsProperty::Product(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for OwnsProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "ownership-info-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<OwnershipInfo as Deserialize>::deserialize(deserializer),
				OwnsProperty::OwnershipInfo,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "product-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Product as Deserialize>::deserialize(deserializer),
				OwnsProperty::Product,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property owns",
			))
		}
	}
}
