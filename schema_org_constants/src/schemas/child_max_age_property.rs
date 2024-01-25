/// <https://schema.org/childMaxAge>
pub const CHILD_MAX_AGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/childMaxAge";
/// <https://schema.org/childMaxAge>
pub const CHILD_MAX_AGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/childMaxAge";
/// <https://schema.org/childMaxAge>
pub const CHILD_MAX_AGE_PROPERTY_LABEL: &str = "childMaxAge";
pub struct ChildMaxAgePropertyIri;
impl PartialEq<&str> for ChildMaxAgePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHILD_MAX_AGE_PROPERTY_IRI_HTTP || *other == CHILD_MAX_AGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ChildMaxAgePropertyIri> for &str {
	fn eq(&self, other: &ChildMaxAgePropertyIri) -> bool {
		*self == CHILD_MAX_AGE_PROPERTY_IRI_HTTP || *self == CHILD_MAX_AGE_PROPERTY_IRI_HTTPS
	}
}
pub struct ChildMaxAgePropertyIriOrLabel;
impl PartialEq<&str> for ChildMaxAgePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ChildMaxAgePropertyIri || *other == CHILD_MAX_AGE_PROPERTY_LABEL
	}
}
impl PartialEq<ChildMaxAgePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ChildMaxAgePropertyIriOrLabel) -> bool {
		*self == ChildMaxAgePropertyIri || *self == CHILD_MAX_AGE_PROPERTY_LABEL
	}
}
