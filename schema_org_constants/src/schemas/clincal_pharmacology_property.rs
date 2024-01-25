/// <https://schema.org/clincalPharmacology>
#[deprecated = "This schema is superseded by <https://schema.org/clinicalPharmacology>."]
pub const CLINCAL_PHARMACOLOGY_PROPERTY_IRI_HTTP: &str = "http://schema.org/clincalPharmacology";
/// <https://schema.org/clincalPharmacology>
#[deprecated = "This schema is superseded by <https://schema.org/clinicalPharmacology>."]
pub const CLINCAL_PHARMACOLOGY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/clincalPharmacology";
/// <https://schema.org/clincalPharmacology>
#[deprecated = "This schema is superseded by <https://schema.org/clinicalPharmacology>."]
pub const CLINCAL_PHARMACOLOGY_PROPERTY_LABEL: &str = "clincalPharmacology";
pub struct ClincalPharmacologyPropertyIri;
impl PartialEq<&str> for ClincalPharmacologyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CLINCAL_PHARMACOLOGY_PROPERTY_IRI_HTTP
			|| *other == CLINCAL_PHARMACOLOGY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ClincalPharmacologyPropertyIri> for &str {
	fn eq(&self, other: &ClincalPharmacologyPropertyIri) -> bool {
		*self == CLINCAL_PHARMACOLOGY_PROPERTY_IRI_HTTP
			|| *self == CLINCAL_PHARMACOLOGY_PROPERTY_IRI_HTTPS
	}
}
pub struct ClincalPharmacologyPropertyIriOrLabel;
impl PartialEq<&str> for ClincalPharmacologyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ClincalPharmacologyPropertyIri || *other == CLINCAL_PHARMACOLOGY_PROPERTY_LABEL
	}
}
impl PartialEq<ClincalPharmacologyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ClincalPharmacologyPropertyIriOrLabel) -> bool {
		*self == ClincalPharmacologyPropertyIri || *self == CLINCAL_PHARMACOLOGY_PROPERTY_LABEL
	}
}
