/// <https://schema.org/educationalRole>
pub const EDUCATIONAL_ROLE_PROPERTY_IRI_HTTP: &str = "http://schema.org/educationalRole";
/// <https://schema.org/educationalRole>
pub const EDUCATIONAL_ROLE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/educationalRole";
/// <https://schema.org/educationalRole>
pub const EDUCATIONAL_ROLE_PROPERTY_LABEL: &str = "educationalRole";
pub struct EducationalRolePropertyIri;
impl PartialEq<&str> for EducationalRolePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EDUCATIONAL_ROLE_PROPERTY_IRI_HTTP
			|| *other == EDUCATIONAL_ROLE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EducationalRolePropertyIri> for &str {
	fn eq(&self, other: &EducationalRolePropertyIri) -> bool {
		*self == EDUCATIONAL_ROLE_PROPERTY_IRI_HTTP || *self == EDUCATIONAL_ROLE_PROPERTY_IRI_HTTPS
	}
}
pub struct EducationalRolePropertyIriOrLabel;
impl PartialEq<&str> for EducationalRolePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EducationalRolePropertyIri || *other == EDUCATIONAL_ROLE_PROPERTY_LABEL
	}
}
impl PartialEq<EducationalRolePropertyIriOrLabel> for &str {
	fn eq(&self, other: &EducationalRolePropertyIriOrLabel) -> bool {
		*self == EducationalRolePropertyIri || *self == EDUCATIONAL_ROLE_PROPERTY_LABEL
	}
}
