/// <https://schema.org/Plumber>
pub trait FindPlumberIds {
	type IdType;
	/// <https://schema.org/Plumber>
	fn find_plumber_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPlumberIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_plumber_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PLUMBER_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PLUMBER_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPlumberIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_plumber_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PLUMBER_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PLUMBER_IRI_HTTPS,
			})
		}
	}
}
