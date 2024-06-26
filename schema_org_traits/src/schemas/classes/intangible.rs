/// <https://schema.org/Intangible>
pub trait FindIntangibleIds {
	type IdType;
	/// <https://schema.org/Intangible>
	fn find_intangible_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindIntangibleIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_intangible_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::INTANGIBLE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::INTANGIBLE_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindIntangibleIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_intangible_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::INTANGIBLE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::INTANGIBLE_IRI_HTTPS,
			})
		}
	}
}
