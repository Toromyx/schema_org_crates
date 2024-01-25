/// <https://schema.org/epidemiology>
pub const EPIDEMIOLOGY_PROPERTY_IRI_HTTP: &str = "http://schema.org/epidemiology";
/// <https://schema.org/epidemiology>
pub const EPIDEMIOLOGY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/epidemiology";
/// <https://schema.org/epidemiology>
pub const EPIDEMIOLOGY_PROPERTY_LABEL: &str = "epidemiology";
pub struct EpidemiologyPropertyIri;
impl PartialEq<&str> for EpidemiologyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EPIDEMIOLOGY_PROPERTY_IRI_HTTP || *other == EPIDEMIOLOGY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EpidemiologyPropertyIri> for &str {
	fn eq(&self, other: &EpidemiologyPropertyIri) -> bool {
		*self == EPIDEMIOLOGY_PROPERTY_IRI_HTTP || *self == EPIDEMIOLOGY_PROPERTY_IRI_HTTPS
	}
}
pub struct EpidemiologyPropertyIriOrLabel;
impl PartialEq<&str> for EpidemiologyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EpidemiologyPropertyIri || *other == EPIDEMIOLOGY_PROPERTY_LABEL
	}
}
impl PartialEq<EpidemiologyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EpidemiologyPropertyIriOrLabel) -> bool {
		*self == EpidemiologyPropertyIri || *self == EPIDEMIOLOGY_PROPERTY_LABEL
	}
}
