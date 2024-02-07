/// <https://schema.org/MotorizedBicycle>
pub trait FindMotorizedBicycleIds {
	type IdType;
	/// <https://schema.org/MotorizedBicycle>
	fn find_motorized_bicycle_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMotorizedBicycleIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_motorized_bicycle_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MOTORIZED_BICYCLE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MOTORIZED_BICYCLE_IRI_HTTPS,
			})
		}
	}
}
