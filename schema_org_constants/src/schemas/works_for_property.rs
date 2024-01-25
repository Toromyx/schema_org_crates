/// <https://schema.org/worksFor>
pub const WORKS_FOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/worksFor";
/// <https://schema.org/worksFor>
pub const WORKS_FOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/worksFor";
/// <https://schema.org/worksFor>
pub const WORKS_FOR_PROPERTY_LABEL: &str = "worksFor";
pub struct WorksForPropertyIri;
impl PartialEq<&str> for WorksForPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WORKS_FOR_PROPERTY_IRI_HTTP || *other == WORKS_FOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WorksForPropertyIri> for &str {
	fn eq(&self, other: &WorksForPropertyIri) -> bool {
		*self == WORKS_FOR_PROPERTY_IRI_HTTP || *self == WORKS_FOR_PROPERTY_IRI_HTTPS
	}
}
pub struct WorksForPropertyIriOrLabel;
impl PartialEq<&str> for WorksForPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WorksForPropertyIri || *other == WORKS_FOR_PROPERTY_LABEL
	}
}
impl PartialEq<WorksForPropertyIriOrLabel> for &str {
	fn eq(&self, other: &WorksForPropertyIriOrLabel) -> bool {
		*self == WorksForPropertyIri || *self == WORKS_FOR_PROPERTY_LABEL
	}
}
