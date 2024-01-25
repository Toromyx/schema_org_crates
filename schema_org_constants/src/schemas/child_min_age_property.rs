/// <https://schema.org/childMinAge>
pub const CHILD_MIN_AGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/childMinAge";
/// <https://schema.org/childMinAge>
pub const CHILD_MIN_AGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/childMinAge";
/// <https://schema.org/childMinAge>
pub const CHILD_MIN_AGE_PROPERTY_LABEL: &str = "childMinAge";
pub struct ChildMinAgePropertyIri;
impl PartialEq<&str> for ChildMinAgePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHILD_MIN_AGE_PROPERTY_IRI_HTTP || *other == CHILD_MIN_AGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ChildMinAgePropertyIri> for &str {
	fn eq(&self, other: &ChildMinAgePropertyIri) -> bool {
		*self == CHILD_MIN_AGE_PROPERTY_IRI_HTTP || *self == CHILD_MIN_AGE_PROPERTY_IRI_HTTPS
	}
}
pub struct ChildMinAgePropertyIriOrLabel;
impl PartialEq<&str> for ChildMinAgePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ChildMinAgePropertyIri || *other == CHILD_MIN_AGE_PROPERTY_LABEL
	}
}
impl PartialEq<ChildMinAgePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ChildMinAgePropertyIriOrLabel) -> bool {
		*self == ChildMinAgePropertyIri || *self == CHILD_MIN_AGE_PROPERTY_LABEL
	}
}
