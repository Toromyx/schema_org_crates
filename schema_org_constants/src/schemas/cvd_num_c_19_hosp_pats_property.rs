/// <https://schema.org/cvdNumC19HospPats>
pub const CVD_NUM_C_19_HOSP_PATS_PROPERTY_IRI_HTTP: &str = "http://schema.org/cvdNumC19HospPats";
/// <https://schema.org/cvdNumC19HospPats>
pub const CVD_NUM_C_19_HOSP_PATS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/cvdNumC19HospPats";
/// <https://schema.org/cvdNumC19HospPats>
pub const CVD_NUM_C_19_HOSP_PATS_PROPERTY_LABEL: &str = "cvdNumC19HospPats";
pub struct CvdNumC19HospPatsPropertyIri;
impl PartialEq<&str> for CvdNumC19HospPatsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CVD_NUM_C_19_HOSP_PATS_PROPERTY_IRI_HTTP
			|| *other == CVD_NUM_C_19_HOSP_PATS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CvdNumC19HospPatsPropertyIri> for &str {
	fn eq(&self, other: &CvdNumC19HospPatsPropertyIri) -> bool {
		*self == CVD_NUM_C_19_HOSP_PATS_PROPERTY_IRI_HTTP
			|| *self == CVD_NUM_C_19_HOSP_PATS_PROPERTY_IRI_HTTPS
	}
}
pub struct CvdNumC19HospPatsPropertyIriOrLabel;
impl PartialEq<&str> for CvdNumC19HospPatsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CvdNumC19HospPatsPropertyIri || *other == CVD_NUM_C_19_HOSP_PATS_PROPERTY_LABEL
	}
}
impl PartialEq<CvdNumC19HospPatsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CvdNumC19HospPatsPropertyIriOrLabel) -> bool {
		*self == CvdNumC19HospPatsPropertyIri || *self == CVD_NUM_C_19_HOSP_PATS_PROPERTY_LABEL
	}
}
