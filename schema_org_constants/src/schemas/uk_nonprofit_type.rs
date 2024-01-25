/// <https://schema.org/UKNonprofitType>
pub const UK_NONPROFIT_TYPE_IRI_HTTP: &str = "http://schema.org/UKNonprofitType";
/// <https://schema.org/UKNonprofitType>
pub const UK_NONPROFIT_TYPE_IRI_HTTPS: &str = "https://schema.org/UKNonprofitType";
/// <https://schema.org/UKNonprofitType>
pub const UK_NONPROFIT_TYPE_LABEL: &str = "UKNonprofitType";
pub struct UkNonprofitTypeIri;
impl PartialEq<&str> for UkNonprofitTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == UK_NONPROFIT_TYPE_IRI_HTTP || *other == UK_NONPROFIT_TYPE_IRI_HTTPS
	}
}
impl PartialEq<UkNonprofitTypeIri> for &str {
	fn eq(&self, other: &UkNonprofitTypeIri) -> bool {
		*self == UK_NONPROFIT_TYPE_IRI_HTTP || *self == UK_NONPROFIT_TYPE_IRI_HTTPS
	}
}
pub struct UkNonprofitTypeIriOrLabel;
impl PartialEq<&str> for UkNonprofitTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UkNonprofitTypeIri || *other == UK_NONPROFIT_TYPE_LABEL
	}
}
impl PartialEq<UkNonprofitTypeIriOrLabel> for &str {
	fn eq(&self, other: &UkNonprofitTypeIriOrLabel) -> bool {
		*self == UkNonprofitTypeIri || *self == UK_NONPROFIT_TYPE_LABEL
	}
}
