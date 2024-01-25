/// <https://schema.org/dissolutionDate>
pub const DISSOLUTION_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/dissolutionDate";
/// <https://schema.org/dissolutionDate>
pub const DISSOLUTION_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/dissolutionDate";
/// <https://schema.org/dissolutionDate>
pub const DISSOLUTION_DATE_PROPERTY_LABEL: &str = "dissolutionDate";
pub struct DissolutionDatePropertyIri;
impl PartialEq<&str> for DissolutionDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DISSOLUTION_DATE_PROPERTY_IRI_HTTP
			|| *other == DISSOLUTION_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DissolutionDatePropertyIri> for &str {
	fn eq(&self, other: &DissolutionDatePropertyIri) -> bool {
		*self == DISSOLUTION_DATE_PROPERTY_IRI_HTTP || *self == DISSOLUTION_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct DissolutionDatePropertyIriOrLabel;
impl PartialEq<&str> for DissolutionDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DissolutionDatePropertyIri || *other == DISSOLUTION_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<DissolutionDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &DissolutionDatePropertyIriOrLabel) -> bool {
		*self == DissolutionDatePropertyIri || *self == DISSOLUTION_DATE_PROPERTY_LABEL
	}
}
