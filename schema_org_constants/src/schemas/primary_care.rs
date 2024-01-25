/// <https://schema.org/PrimaryCare>
pub const PRIMARY_CARE_IRI_HTTP: &str = "http://schema.org/PrimaryCare";
/// <https://schema.org/PrimaryCare>
pub const PRIMARY_CARE_IRI_HTTPS: &str = "https://schema.org/PrimaryCare";
/// <https://schema.org/PrimaryCare>
pub const PRIMARY_CARE_LABEL: &str = "PrimaryCare";
pub struct PrimaryCareIri;
impl PartialEq<&str> for PrimaryCareIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRIMARY_CARE_IRI_HTTP || *other == PRIMARY_CARE_IRI_HTTPS
	}
}
impl PartialEq<PrimaryCareIri> for &str {
	fn eq(&self, other: &PrimaryCareIri) -> bool {
		*self == PRIMARY_CARE_IRI_HTTP || *self == PRIMARY_CARE_IRI_HTTPS
	}
}
pub struct PrimaryCareIriOrLabel;
impl PartialEq<&str> for PrimaryCareIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PrimaryCareIri || *other == PRIMARY_CARE_LABEL
	}
}
impl PartialEq<PrimaryCareIriOrLabel> for &str {
	fn eq(&self, other: &PrimaryCareIriOrLabel) -> bool {
		*self == PrimaryCareIri || *self == PRIMARY_CARE_LABEL
	}
}
