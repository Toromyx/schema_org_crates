/// <https://schema.org/AutomatedTeller>
pub trait FindAutomatedTellerIds {
	type IdType;
	/// <https://schema.org/AutomatedTeller>
	fn find_automated_teller_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindAutomatedTellerIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_automated_teller_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::AUTOMATED_TELLER_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::AUTOMATED_TELLER_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindAutomatedTellerIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_automated_teller_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::AUTOMATED_TELLER_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::AUTOMATED_TELLER_IRI_HTTPS,
			})
		}
	}
}
