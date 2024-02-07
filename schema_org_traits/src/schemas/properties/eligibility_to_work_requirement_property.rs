/// <https://schema.org/eligibilityToWorkRequirement>
pub trait GetEligibilityToWorkRequirementProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/eligibilityToWorkRequirement>
	fn get_eligibility_to_work_requirement_property(
		&self,
		id: &Self::IdType,
	) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetEligibilityToWorkRequirementProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_eligibility_to_work_requirement_property(
			&self,
			id: &Self::IdType,
		) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => {
						schema_org_constants::ELIGIBILITY_TO_WORK_REQUIREMENT_PROPERTY_IRI_HTTP
					}
					SchemaOrgNamespace::Https => {
						schema_org_constants::ELIGIBILITY_TO_WORK_REQUIREMENT_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
