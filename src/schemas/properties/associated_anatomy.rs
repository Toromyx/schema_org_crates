use super::*;
/// <https://schema.org/associatedAnatomy>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum AssociatedAnatomyProperty {
	#[cfg(any(
		any(
			feature = "anatomical-structure-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	AnatomicalStructure(AnatomicalStructure),
	#[cfg(any(
		any(
			feature = "anatomical-system-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	AnatomicalSystem(AnatomicalSystem),
	#[cfg(any(
		any(
			feature = "superficial-anatomy-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	SuperficialAnatomy(SuperficialAnatomy),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for AssociatedAnatomyProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(
						feature = "anatomical-structure-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				AssociatedAnatomyProperty::AnatomicalStructure(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(
						feature = "anatomical-system-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				AssociatedAnatomyProperty::AnatomicalSystem(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(
						feature = "superficial-anatomy-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				AssociatedAnatomyProperty::SuperficialAnatomy(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for AssociatedAnatomyProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(
					feature = "anatomical-structure-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<AnatomicalStructure as Deserialize>::deserialize(deserializer),
				AssociatedAnatomyProperty::AnatomicalStructure,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(
					feature = "anatomical-system-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<AnatomicalSystem as Deserialize>::deserialize(deserializer),
				AssociatedAnatomyProperty::AnatomicalSystem,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(
					feature = "superficial-anatomy-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<SuperficialAnatomy as Deserialize>::deserialize(deserializer),
				AssociatedAnatomyProperty::SuperficialAnatomy,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property associatedAnatomy",
			))
		}
	}
}
