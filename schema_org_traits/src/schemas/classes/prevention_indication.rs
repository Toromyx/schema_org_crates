/// <https://schema.org/PreventionIndication>
pub trait FindPreventionIndicationIds {
	type IdType;
	fn find_prevention_indication_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPreventionIndicationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_prevention_indication_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PREVENTION_INDICATION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PREVENTION_INDICATION_IRI_HTTPS,
			})
		}
	}
}
