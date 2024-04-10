/// <https://schema.org/Dermatologic>
#[deprecated = "This schema is superseded by <https://schema.org/Dermatology>."]
pub trait FindDermatologicIds {
	type IdType;
	/// <https://schema.org/Dermatologic>
	#[deprecated = "This schema is superseded by <https://schema.org/Dermatology>."]
	fn find_dermatologic_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDermatologicIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_dermatologic_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DERMATOLOGIC_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DERMATOLOGIC_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDermatologicIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_dermatologic_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DERMATOLOGIC_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DERMATOLOGIC_IRI_HTTPS,
			})
		}
	}
}
