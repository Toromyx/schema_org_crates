/// <https://schema.org/Rheumatologic>
pub const RHEUMATOLOGIC_IRI_HTTP: &str = "http://schema.org/Rheumatologic";
/// <https://schema.org/Rheumatologic>
pub const RHEUMATOLOGIC_IRI_HTTPS: &str = "https://schema.org/Rheumatologic";
/// <https://schema.org/Rheumatologic>
pub const RHEUMATOLOGIC_LABEL: &str = "Rheumatologic";
pub struct RheumatologicIri;
impl PartialEq<&str> for RheumatologicIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RHEUMATOLOGIC_IRI_HTTP || *other == RHEUMATOLOGIC_IRI_HTTPS
	}
}
impl PartialEq<RheumatologicIri> for &str {
	fn eq(&self, other: &RheumatologicIri) -> bool {
		*self == RHEUMATOLOGIC_IRI_HTTP || *self == RHEUMATOLOGIC_IRI_HTTPS
	}
}
pub struct RheumatologicIriOrLabel;
impl PartialEq<&str> for RheumatologicIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RheumatologicIri || *other == RHEUMATOLOGIC_LABEL
	}
}
impl PartialEq<RheumatologicIriOrLabel> for &str {
	fn eq(&self, other: &RheumatologicIriOrLabel) -> bool {
		*self == RheumatologicIri || *self == RHEUMATOLOGIC_LABEL
	}
}
