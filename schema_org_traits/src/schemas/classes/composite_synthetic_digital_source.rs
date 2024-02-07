/// <https://schema.org/CompositeSyntheticDigitalSource>
pub trait FindCompositeSyntheticDigitalSourceIds {
	type IdType;
	/// <https://schema.org/CompositeSyntheticDigitalSource>
	fn find_composite_synthetic_digital_source_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCompositeSyntheticDigitalSourceIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_composite_synthetic_digital_source_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::COMPOSITE_SYNTHETIC_DIGITAL_SOURCE_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::COMPOSITE_SYNTHETIC_DIGITAL_SOURCE_IRI_HTTPS
				}
			})
		}
	}
}
