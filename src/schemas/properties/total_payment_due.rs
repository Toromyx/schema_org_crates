use super::*;
/// <https://schema.org/totalPaymentDue>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum TotalPaymentDueProperty {
	#[cfg(any(
		any(feature = "monetary-amount-schema", feature = "general-schema-section"),
		doc
	))]
	MonetaryAmount(MonetaryAmount),
	#[cfg(any(
		any(
			feature = "price-specification-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	PriceSpecification(PriceSpecification),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for TotalPaymentDueProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "monetary-amount-schema", feature = "general-schema-section"),
					doc
				))]
				TotalPaymentDueProperty::MonetaryAmount(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(
						feature = "price-specification-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				TotalPaymentDueProperty::PriceSpecification(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for TotalPaymentDueProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "monetary-amount-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<MonetaryAmount as Deserialize>::deserialize(deserializer),
				TotalPaymentDueProperty::MonetaryAmount,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(
					feature = "price-specification-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<PriceSpecification as Deserialize>::deserialize(deserializer),
				TotalPaymentDueProperty::PriceSpecification,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property totalPaymentDue",
			))
		}
	}
}
