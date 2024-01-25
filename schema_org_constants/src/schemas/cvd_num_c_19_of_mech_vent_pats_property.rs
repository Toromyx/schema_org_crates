/// <https://schema.org/cvdNumC19OFMechVentPats>
pub const CVD_NUM_C_19_OF_MECH_VENT_PATS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/cvdNumC19OFMechVentPats";
/// <https://schema.org/cvdNumC19OFMechVentPats>
pub const CVD_NUM_C_19_OF_MECH_VENT_PATS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/cvdNumC19OFMechVentPats";
/// <https://schema.org/cvdNumC19OFMechVentPats>
pub const CVD_NUM_C_19_OF_MECH_VENT_PATS_PROPERTY_LABEL: &str = "cvdNumC19OFMechVentPats";
pub struct CvdNumC19OfMechVentPatsPropertyIri;
impl PartialEq<&str> for CvdNumC19OfMechVentPatsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CVD_NUM_C_19_OF_MECH_VENT_PATS_PROPERTY_IRI_HTTP
			|| *other == CVD_NUM_C_19_OF_MECH_VENT_PATS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CvdNumC19OfMechVentPatsPropertyIri> for &str {
	fn eq(&self, other: &CvdNumC19OfMechVentPatsPropertyIri) -> bool {
		*self == CVD_NUM_C_19_OF_MECH_VENT_PATS_PROPERTY_IRI_HTTP
			|| *self == CVD_NUM_C_19_OF_MECH_VENT_PATS_PROPERTY_IRI_HTTPS
	}
}
pub struct CvdNumC19OfMechVentPatsPropertyIriOrLabel;
impl PartialEq<&str> for CvdNumC19OfMechVentPatsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CvdNumC19OfMechVentPatsPropertyIri
			|| *other == CVD_NUM_C_19_OF_MECH_VENT_PATS_PROPERTY_LABEL
	}
}
impl PartialEq<CvdNumC19OfMechVentPatsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CvdNumC19OfMechVentPatsPropertyIriOrLabel) -> bool {
		*self == CvdNumC19OfMechVentPatsPropertyIri
			|| *self == CVD_NUM_C_19_OF_MECH_VENT_PATS_PROPERTY_LABEL
	}
}
