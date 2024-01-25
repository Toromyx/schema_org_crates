/// <https://schema.org/MSRP>
pub const MSRP_IRI_HTTP: &str = "http://schema.org/MSRP";
/// <https://schema.org/MSRP>
pub const MSRP_IRI_HTTPS: &str = "https://schema.org/MSRP";
/// <https://schema.org/MSRP>
pub const MSRP_LABEL: &str = "MSRP";
pub struct MsrpIri;
impl PartialEq<&str> for MsrpIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MSRP_IRI_HTTP || *other == MSRP_IRI_HTTPS
	}
}
impl PartialEq<MsrpIri> for &str {
	fn eq(&self, other: &MsrpIri) -> bool {
		*self == MSRP_IRI_HTTP || *self == MSRP_IRI_HTTPS
	}
}
pub struct MsrpIriOrLabel;
impl PartialEq<&str> for MsrpIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MsrpIri || *other == MSRP_LABEL
	}
}
impl PartialEq<MsrpIriOrLabel> for &str {
	fn eq(&self, other: &MsrpIriOrLabel) -> bool {
		*self == MsrpIri || *self == MSRP_LABEL
	}
}
