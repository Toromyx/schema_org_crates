/// <https://schema.org/CrossSectional>
pub trait FindCrossSectionalIds {
	type IdType;
	/// <https://schema.org/CrossSectional>
	fn find_cross_sectional_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCrossSectionalIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_cross_sectional_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CROSS_SECTIONAL_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CROSS_SECTIONAL_IRI_HTTPS,
			})
		}
	}
}
