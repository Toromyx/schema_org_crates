/// <https://schema.org/nonprofitStatus>
pub const NONPROFIT_STATUS_PROPERTY_IRI_HTTP: &str = "http://schema.org/nonprofitStatus";
/// <https://schema.org/nonprofitStatus>
pub const NONPROFIT_STATUS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/nonprofitStatus";
/// <https://schema.org/nonprofitStatus>
pub const NONPROFIT_STATUS_PROPERTY_LABEL: &str = "nonprofitStatus";
pub struct NonprofitStatusPropertyIri;
impl PartialEq<&str> for NonprofitStatusPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_STATUS_PROPERTY_IRI_HTTP
			|| *other == NONPROFIT_STATUS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NonprofitStatusPropertyIri> for &str {
	fn eq(&self, other: &NonprofitStatusPropertyIri) -> bool {
		*self == NONPROFIT_STATUS_PROPERTY_IRI_HTTP || *self == NONPROFIT_STATUS_PROPERTY_IRI_HTTPS
	}
}
pub struct NonprofitStatusPropertyIriOrLabel;
impl PartialEq<&str> for NonprofitStatusPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NonprofitStatusPropertyIri || *other == NONPROFIT_STATUS_PROPERTY_LABEL
	}
}
impl PartialEq<NonprofitStatusPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NonprofitStatusPropertyIriOrLabel) -> bool {
		*self == NonprofitStatusPropertyIri || *self == NONPROFIT_STATUS_PROPERTY_LABEL
	}
}
