/// <https://schema.org/UnemploymentSupport>
pub const UNEMPLOYMENT_SUPPORT_IRI_HTTP: &str = "http://schema.org/UnemploymentSupport";
/// <https://schema.org/UnemploymentSupport>
pub const UNEMPLOYMENT_SUPPORT_IRI_HTTPS: &str = "https://schema.org/UnemploymentSupport";
/// <https://schema.org/UnemploymentSupport>
pub const UNEMPLOYMENT_SUPPORT_LABEL: &str = "UnemploymentSupport";
pub struct UnemploymentSupportIri;
impl PartialEq<&str> for UnemploymentSupportIri {
	fn eq(&self, other: &&str) -> bool {
		*other == UNEMPLOYMENT_SUPPORT_IRI_HTTP || *other == UNEMPLOYMENT_SUPPORT_IRI_HTTPS
	}
}
impl PartialEq<UnemploymentSupportIri> for &str {
	fn eq(&self, other: &UnemploymentSupportIri) -> bool {
		*self == UNEMPLOYMENT_SUPPORT_IRI_HTTP || *self == UNEMPLOYMENT_SUPPORT_IRI_HTTPS
	}
}
pub struct UnemploymentSupportIriOrLabel;
impl PartialEq<&str> for UnemploymentSupportIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UnemploymentSupportIri || *other == UNEMPLOYMENT_SUPPORT_LABEL
	}
}
impl PartialEq<UnemploymentSupportIriOrLabel> for &str {
	fn eq(&self, other: &UnemploymentSupportIriOrLabel) -> bool {
		*self == UnemploymentSupportIri || *self == UNEMPLOYMENT_SUPPORT_LABEL
	}
}
