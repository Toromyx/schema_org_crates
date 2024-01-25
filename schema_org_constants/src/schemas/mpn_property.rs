/// <https://schema.org/mpn>
pub const MPN_PROPERTY_IRI_HTTP: &str = "http://schema.org/mpn";
/// <https://schema.org/mpn>
pub const MPN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/mpn";
/// <https://schema.org/mpn>
pub const MPN_PROPERTY_LABEL: &str = "mpn";
pub struct MpnPropertyIri;
impl PartialEq<&str> for MpnPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MPN_PROPERTY_IRI_HTTP || *other == MPN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MpnPropertyIri> for &str {
	fn eq(&self, other: &MpnPropertyIri) -> bool {
		*self == MPN_PROPERTY_IRI_HTTP || *self == MPN_PROPERTY_IRI_HTTPS
	}
}
pub struct MpnPropertyIriOrLabel;
impl PartialEq<&str> for MpnPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MpnPropertyIri || *other == MPN_PROPERTY_LABEL
	}
}
impl PartialEq<MpnPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MpnPropertyIriOrLabel) -> bool {
		*self == MpnPropertyIri || *self == MPN_PROPERTY_LABEL
	}
}
