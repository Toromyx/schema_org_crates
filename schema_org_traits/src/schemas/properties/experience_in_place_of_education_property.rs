/// <https://schema.org/experienceInPlaceOfEducation>
pub trait GetExperienceInPlaceOfEducationProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/experienceInPlaceOfEducation>
	fn get_experience_in_place_of_education_property(
		&self,
		id: &Self::IdType,
	) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetExperienceInPlaceOfEducationProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_experience_in_place_of_education_property(
			&self,
			id: &Self::IdType,
		) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => {
						schema_org_constants::EXPERIENCE_IN_PLACE_OF_EDUCATION_PROPERTY_IRI_HTTP
					}
					SchemaOrgNamespace::Https => {
						schema_org_constants::EXPERIENCE_IN_PLACE_OF_EDUCATION_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
