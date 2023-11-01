use std::str::FromStr;

use convert_case::{Case, Casing};
use quote::{__private::TokenStream, quote};

use crate::{feature::Feature, schema::property::Property};

pub fn serde_mod(property: &Property) -> TokenStream {
	let schema_name = &property.name;
	let name_string = format!("{}Property", schema_name.to_case(Case::UpperCamel));
	let name = TokenStream::from_str(&name_string).unwrap();
	let deserialize_custom_error = format!(
		"data did not match any variant of schema.org property {}",
		schema_name
	);

	struct VariantTokenStreams {
		variant_name: TokenStream,
		feature_gate: TokenStream,
	}

	let variant_token_streams: Vec<VariantTokenStreams> = property
		.variants
		.iter()
		.map(|referenced_schema| {
			let variant_name =
				TokenStream::from_str(&referenced_schema.name.to_case(Case::UpperCamel)).unwrap();
			let feature = Feature::Any(vec![
				Feature::Name(format!(
					"{}-schema",
					referenced_schema.name.to_case(Case::Kebab)
				)),
				Feature::Name(referenced_schema.section.feature_name().to_string()),
			]);
			let feature_gate = feature.as_cfg_attribute();
			VariantTokenStreams {
				variant_name,
				feature_gate,
			}
		})
		.collect();

	let serialize_match_arms = variant_token_streams.iter().map(|token_streams| {
		let variant_name = &token_streams.variant_name;
		let feature_gate = &token_streams.feature_gate;
		quote!(
			#feature_gate
			#name::#variant_name(ref inner) => inner.serialize(serializer),
		)
	});

	let deserialize_variants = variant_token_streams.iter().map(|token_streams| {
		let variant_name = &token_streams.variant_name;
		let feature_gate = &token_streams.feature_gate;
		quote!(
			#feature_gate
			if let Ok(ok) = Result::map(
				<#variant_name as Deserialize>::deserialize(deserializer),
				#name::#variant_name,
			) {
				return Ok(ok);
			}
		)
	});

	quote!(
		use std::{fmt, fmt::Formatter};

		use ::serde::{de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

		use super::*;

		impl Serialize for #name {
			fn serialize<S>(
				&self,
				serializer: S,
			) -> Result<S::Ok, S::Error>
			where
				S: Serializer,
			{
				match *self {
					#(#serialize_match_arms)*
				}
			}
		}

		impl<'de> Deserialize<'de> for #name {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: Deserializer<'de>,
			{
				let content = <::serde::__private::de::Content as Deserialize>::deserialize(
					deserializer,
				)?;
				let deserializer = ::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
				#(#deserialize_variants)*
				Err(de::Error::custom(#deserialize_custom_error))
			}
		}
	)
}
