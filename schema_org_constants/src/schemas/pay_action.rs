/// <https://schema.org/PayAction>
pub const PAY_ACTION_IRI_HTTP: &str = "http://schema.org/PayAction";
/// <https://schema.org/PayAction>
pub const PAY_ACTION_IRI_HTTPS: &str = "https://schema.org/PayAction";
/// <https://schema.org/PayAction>
pub const PAY_ACTION_LABEL: &str = "PayAction";
pub struct PayActionIri;
impl PartialEq<&str> for PayActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAY_ACTION_IRI_HTTP || *other == PAY_ACTION_IRI_HTTPS
	}
}
impl PartialEq<PayActionIri> for &str {
	fn eq(&self, other: &PayActionIri) -> bool {
		*self == PAY_ACTION_IRI_HTTP || *self == PAY_ACTION_IRI_HTTPS
	}
}
pub struct PayActionIriOrLabel;
impl PartialEq<&str> for PayActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PayActionIri || *other == PAY_ACTION_LABEL
	}
}
impl PartialEq<PayActionIriOrLabel> for &str {
	fn eq(&self, other: &PayActionIriOrLabel) -> bool {
		*self == PayActionIri || *self == PAY_ACTION_LABEL
	}
}
