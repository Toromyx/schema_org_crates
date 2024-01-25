/// <https://schema.org/cvdNumC19MechVentPats>
pub const CVD_NUM_C_19_MECH_VENT_PATS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/cvdNumC19MechVentPats";
/// <https://schema.org/cvdNumC19MechVentPats>
pub const CVD_NUM_C_19_MECH_VENT_PATS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/cvdNumC19MechVentPats";
/// <https://schema.org/cvdNumC19MechVentPats>
pub const CVD_NUM_C_19_MECH_VENT_PATS_PROPERTY_LABEL: &str = "cvdNumC19MechVentPats";
pub struct CvdNumC19MechVentPatsPropertyIri;
impl PartialEq<&str> for CvdNumC19MechVentPatsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CVD_NUM_C_19_MECH_VENT_PATS_PROPERTY_IRI_HTTP
			|| *other == CVD_NUM_C_19_MECH_VENT_PATS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CvdNumC19MechVentPatsPropertyIri> for &str {
	fn eq(&self, other: &CvdNumC19MechVentPatsPropertyIri) -> bool {
		*self == CVD_NUM_C_19_MECH_VENT_PATS_PROPERTY_IRI_HTTP
			|| *self == CVD_NUM_C_19_MECH_VENT_PATS_PROPERTY_IRI_HTTPS
	}
}
pub struct CvdNumC19MechVentPatsPropertyIriOrLabel;
impl PartialEq<&str> for CvdNumC19MechVentPatsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CvdNumC19MechVentPatsPropertyIri
			|| *other == CVD_NUM_C_19_MECH_VENT_PATS_PROPERTY_LABEL
	}
}
impl PartialEq<CvdNumC19MechVentPatsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CvdNumC19MechVentPatsPropertyIriOrLabel) -> bool {
		*self == CvdNumC19MechVentPatsPropertyIri
			|| *self == CVD_NUM_C_19_MECH_VENT_PATS_PROPERTY_LABEL
	}
}
