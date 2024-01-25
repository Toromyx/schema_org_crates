/// <https://schema.org/AuthoritativeLegalValue>
pub trait FindAuthoritativeLegalValueIds {
	type IdType;
	fn find_authoritative_legal_value_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindAuthoritativeLegalValueIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_authoritative_legal_value_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::AUTHORITATIVE_LEGAL_VALUE_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::AUTHORITATIVE_LEGAL_VALUE_IRI_HTTPS
				}
			})
		}
	}
}
