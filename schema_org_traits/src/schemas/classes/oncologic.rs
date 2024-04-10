/// <https://schema.org/Oncologic>
pub trait FindOncologicIds {
	type IdType;
	/// <https://schema.org/Oncologic>
	fn find_oncologic_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOncologicIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_oncologic_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ONCOLOGIC_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ONCOLOGIC_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOncologicIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_oncologic_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ONCOLOGIC_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ONCOLOGIC_IRI_HTTPS,
			})
		}
	}
}
