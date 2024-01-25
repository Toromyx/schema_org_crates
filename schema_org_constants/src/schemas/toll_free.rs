/// <https://schema.org/TollFree>
pub const TOLL_FREE_IRI_HTTP: &str = "http://schema.org/TollFree";
/// <https://schema.org/TollFree>
pub const TOLL_FREE_IRI_HTTPS: &str = "https://schema.org/TollFree";
/// <https://schema.org/TollFree>
pub const TOLL_FREE_LABEL: &str = "TollFree";
pub struct TollFreeIri;
impl PartialEq<&str> for TollFreeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TOLL_FREE_IRI_HTTP || *other == TOLL_FREE_IRI_HTTPS
	}
}
impl PartialEq<TollFreeIri> for &str {
	fn eq(&self, other: &TollFreeIri) -> bool {
		*self == TOLL_FREE_IRI_HTTP || *self == TOLL_FREE_IRI_HTTPS
	}
}
pub struct TollFreeIriOrLabel;
impl PartialEq<&str> for TollFreeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TollFreeIri || *other == TOLL_FREE_LABEL
	}
}
impl PartialEq<TollFreeIriOrLabel> for &str {
	fn eq(&self, other: &TollFreeIriOrLabel) -> bool {
		*self == TollFreeIri || *self == TOLL_FREE_LABEL
	}
}
