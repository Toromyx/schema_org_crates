/// <https://schema.org/DonateAction>
pub const DONATE_ACTION_IRI_HTTP: &str = "http://schema.org/DonateAction";
/// <https://schema.org/DonateAction>
pub const DONATE_ACTION_IRI_HTTPS: &str = "https://schema.org/DonateAction";
/// <https://schema.org/DonateAction>
pub const DONATE_ACTION_LABEL: &str = "DonateAction";
pub struct DonateActionIri;
impl PartialEq<&str> for DonateActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DONATE_ACTION_IRI_HTTP || *other == DONATE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<DonateActionIri> for &str {
	fn eq(&self, other: &DonateActionIri) -> bool {
		*self == DONATE_ACTION_IRI_HTTP || *self == DONATE_ACTION_IRI_HTTPS
	}
}
pub struct DonateActionIriOrLabel;
impl PartialEq<&str> for DonateActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DonateActionIri || *other == DONATE_ACTION_LABEL
	}
}
impl PartialEq<DonateActionIriOrLabel> for &str {
	fn eq(&self, other: &DonateActionIriOrLabel) -> bool {
		*self == DonateActionIri || *self == DONATE_ACTION_LABEL
	}
}
