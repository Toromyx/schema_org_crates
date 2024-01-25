/// <https://schema.org/rxcui>
pub const RXCUI_PROPERTY_IRI_HTTP: &str = "http://schema.org/rxcui";
/// <https://schema.org/rxcui>
pub const RXCUI_PROPERTY_IRI_HTTPS: &str = "https://schema.org/rxcui";
/// <https://schema.org/rxcui>
pub const RXCUI_PROPERTY_LABEL: &str = "rxcui";
pub struct RxcuiPropertyIri;
impl PartialEq<&str> for RxcuiPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RXCUI_PROPERTY_IRI_HTTP || *other == RXCUI_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RxcuiPropertyIri> for &str {
	fn eq(&self, other: &RxcuiPropertyIri) -> bool {
		*self == RXCUI_PROPERTY_IRI_HTTP || *self == RXCUI_PROPERTY_IRI_HTTPS
	}
}
pub struct RxcuiPropertyIriOrLabel;
impl PartialEq<&str> for RxcuiPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RxcuiPropertyIri || *other == RXCUI_PROPERTY_LABEL
	}
}
impl PartialEq<RxcuiPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RxcuiPropertyIriOrLabel) -> bool {
		*self == RxcuiPropertyIri || *self == RXCUI_PROPERTY_LABEL
	}
}
