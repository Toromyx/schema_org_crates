/// <https://schema.org/exerciseCourse>
pub trait GetExerciseCourseProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/exerciseCourse>
	fn get_exercise_course_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetExerciseCourseProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_exercise_course_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => {
						schema_org_constants::EXERCISE_COURSE_PROPERTY_IRI_HTTP
					}
					SchemaOrgNamespace::Https => {
						schema_org_constants::EXERCISE_COURSE_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
