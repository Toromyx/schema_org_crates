/// <https://schema.org/cvdNumC19OverflowPats>
pub const CVD_NUM_C_19_OVERFLOW_PATS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/cvdNumC19OverflowPats";
/// <https://schema.org/cvdNumC19OverflowPats>
pub const CVD_NUM_C_19_OVERFLOW_PATS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/cvdNumC19OverflowPats";
/// <https://schema.org/cvdNumC19OverflowPats>
pub const CVD_NUM_C_19_OVERFLOW_PATS_PROPERTY_LABEL: &str = "cvdNumC19OverflowPats";
pub struct CvdNumC19OverflowPatsPropertyIri;
impl PartialEq<&str> for CvdNumC19OverflowPatsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CVD_NUM_C_19_OVERFLOW_PATS_PROPERTY_IRI_HTTP
			|| *other == CVD_NUM_C_19_OVERFLOW_PATS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CvdNumC19OverflowPatsPropertyIri> for &str {
	fn eq(&self, other: &CvdNumC19OverflowPatsPropertyIri) -> bool {
		*self == CVD_NUM_C_19_OVERFLOW_PATS_PROPERTY_IRI_HTTP
			|| *self == CVD_NUM_C_19_OVERFLOW_PATS_PROPERTY_IRI_HTTPS
	}
}
pub struct CvdNumC19OverflowPatsPropertyIriOrLabel;
impl PartialEq<&str> for CvdNumC19OverflowPatsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CvdNumC19OverflowPatsPropertyIri
			|| *other == CVD_NUM_C_19_OVERFLOW_PATS_PROPERTY_LABEL
	}
}
impl PartialEq<CvdNumC19OverflowPatsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CvdNumC19OverflowPatsPropertyIriOrLabel) -> bool {
		*self == CvdNumC19OverflowPatsPropertyIri
			|| *self == CVD_NUM_C_19_OVERFLOW_PATS_PROPERTY_LABEL
	}
}
