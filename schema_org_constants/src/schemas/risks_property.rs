/// <https://schema.org/risks>
pub const RISKS_PROPERTY_IRI_HTTP: &str = "http://schema.org/risks";
/// <https://schema.org/risks>
pub const RISKS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/risks";
/// <https://schema.org/risks>
pub const RISKS_PROPERTY_LABEL: &str = "risks";
pub struct RisksPropertyIri;
impl PartialEq<&str> for RisksPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RISKS_PROPERTY_IRI_HTTP || *other == RISKS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RisksPropertyIri> for &str {
	fn eq(&self, other: &RisksPropertyIri) -> bool {
		*self == RISKS_PROPERTY_IRI_HTTP || *self == RISKS_PROPERTY_IRI_HTTPS
	}
}
pub struct RisksPropertyIriOrLabel;
impl PartialEq<&str> for RisksPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RisksPropertyIri || *other == RISKS_PROPERTY_LABEL
	}
}
impl PartialEq<RisksPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RisksPropertyIriOrLabel) -> bool {
		*self == RisksPropertyIri || *self == RISKS_PROPERTY_LABEL
	}
}
