/// <https://schema.org/sponsor>
pub const SPONSOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/sponsor";
/// <https://schema.org/sponsor>
pub const SPONSOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sponsor";
/// <https://schema.org/sponsor>
pub const SPONSOR_PROPERTY_LABEL: &str = "sponsor";
pub struct SponsorPropertyIri;
impl PartialEq<&str> for SponsorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPONSOR_PROPERTY_IRI_HTTP || *other == SPONSOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SponsorPropertyIri> for &str {
	fn eq(&self, other: &SponsorPropertyIri) -> bool {
		*self == SPONSOR_PROPERTY_IRI_HTTP || *self == SPONSOR_PROPERTY_IRI_HTTPS
	}
}
pub struct SponsorPropertyIriOrLabel;
impl PartialEq<&str> for SponsorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SponsorPropertyIri || *other == SPONSOR_PROPERTY_LABEL
	}
}
impl PartialEq<SponsorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SponsorPropertyIriOrLabel) -> bool {
		*self == SponsorPropertyIri || *self == SPONSOR_PROPERTY_LABEL
	}
}
