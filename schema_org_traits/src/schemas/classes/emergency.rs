/// <https://schema.org/Emergency>
pub trait FindEmergencyIds {
	type IdType;
	/// <https://schema.org/Emergency>
	fn find_emergency_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEmergencyIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_emergency_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::EMERGENCY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::EMERGENCY_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEmergencyIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_emergency_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::EMERGENCY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::EMERGENCY_IRI_HTTPS,
			})
		}
	}
}
