/// <https://schema.org/cvdNumC19HOPats>
pub const CVD_NUM_C_19_HO_PATS_PROPERTY_IRI_HTTP: &str = "http://schema.org/cvdNumC19HOPats";
/// <https://schema.org/cvdNumC19HOPats>
pub const CVD_NUM_C_19_HO_PATS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/cvdNumC19HOPats";
/// <https://schema.org/cvdNumC19HOPats>
pub const CVD_NUM_C_19_HO_PATS_PROPERTY_LABEL: &str = "cvdNumC19HOPats";
pub struct CvdNumC19HoPatsPropertyIri;
impl PartialEq<&str> for CvdNumC19HoPatsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CVD_NUM_C_19_HO_PATS_PROPERTY_IRI_HTTP
			|| *other == CVD_NUM_C_19_HO_PATS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CvdNumC19HoPatsPropertyIri> for &str {
	fn eq(&self, other: &CvdNumC19HoPatsPropertyIri) -> bool {
		*self == CVD_NUM_C_19_HO_PATS_PROPERTY_IRI_HTTP
			|| *self == CVD_NUM_C_19_HO_PATS_PROPERTY_IRI_HTTPS
	}
}
pub struct CvdNumC19HoPatsPropertyIriOrLabel;
impl PartialEq<&str> for CvdNumC19HoPatsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CvdNumC19HoPatsPropertyIri || *other == CVD_NUM_C_19_HO_PATS_PROPERTY_LABEL
	}
}
impl PartialEq<CvdNumC19HoPatsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CvdNumC19HoPatsPropertyIriOrLabel) -> bool {
		*self == CvdNumC19HoPatsPropertyIri || *self == CVD_NUM_C_19_HO_PATS_PROPERTY_LABEL
	}
}
