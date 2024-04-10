/// <https://schema.org/PropertyValue>
pub trait FindPropertyValueIds {
	type IdType;
	/// <https://schema.org/PropertyValue>
	fn find_property_value_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPropertyValueIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_property_value_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PROPERTY_VALUE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PROPERTY_VALUE_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPropertyValueIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_property_value_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PROPERTY_VALUE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PROPERTY_VALUE_IRI_HTTPS,
			})
		}
	}
}
