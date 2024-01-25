/// <https://schema.org/biologicalRole>
pub const BIOLOGICAL_ROLE_PROPERTY_IRI_HTTP: &str = "http://schema.org/biologicalRole";
/// <https://schema.org/biologicalRole>
pub const BIOLOGICAL_ROLE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/biologicalRole";
/// <https://schema.org/biologicalRole>
pub const BIOLOGICAL_ROLE_PROPERTY_LABEL: &str = "biologicalRole";
pub struct BiologicalRolePropertyIri;
impl PartialEq<&str> for BiologicalRolePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BIOLOGICAL_ROLE_PROPERTY_IRI_HTTP || *other == BIOLOGICAL_ROLE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BiologicalRolePropertyIri> for &str {
	fn eq(&self, other: &BiologicalRolePropertyIri) -> bool {
		*self == BIOLOGICAL_ROLE_PROPERTY_IRI_HTTP || *self == BIOLOGICAL_ROLE_PROPERTY_IRI_HTTPS
	}
}
pub struct BiologicalRolePropertyIriOrLabel;
impl PartialEq<&str> for BiologicalRolePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BiologicalRolePropertyIri || *other == BIOLOGICAL_ROLE_PROPERTY_LABEL
	}
}
impl PartialEq<BiologicalRolePropertyIriOrLabel> for &str {
	fn eq(&self, other: &BiologicalRolePropertyIriOrLabel) -> bool {
		*self == BiologicalRolePropertyIri || *self == BIOLOGICAL_ROLE_PROPERTY_LABEL
	}
}
