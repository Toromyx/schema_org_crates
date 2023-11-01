use super::*;
/// <https://schema.org/availableService>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum AvailableServiceProperty {
	#[cfg(any(
		any(
			feature = "medical-procedure-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalProcedure(MedicalProcedure),
	#[cfg(any(
		any(
			feature = "medical-test-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalTest(MedicalTest),
	#[cfg(any(
		any(
			feature = "medical-therapy-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalTherapy(MedicalTherapy),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for AvailableServiceProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(
						feature = "medical-procedure-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				AvailableServiceProperty::MedicalProcedure(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(
						feature = "medical-test-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				AvailableServiceProperty::MedicalTest(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(
						feature = "medical-therapy-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				AvailableServiceProperty::MedicalTherapy(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for AvailableServiceProperty {
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
					feature = "medical-procedure-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<MedicalProcedure as Deserialize>::deserialize(deserializer),
				AvailableServiceProperty::MedicalProcedure,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(
					feature = "medical-test-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<MedicalTest as Deserialize>::deserialize(deserializer),
				AvailableServiceProperty::MedicalTest,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(
					feature = "medical-therapy-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<MedicalTherapy as Deserialize>::deserialize(deserializer),
				AvailableServiceProperty::MedicalTherapy,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property availableService",
			))
		}
	}
}
