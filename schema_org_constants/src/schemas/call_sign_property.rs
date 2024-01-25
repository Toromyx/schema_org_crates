/// <https://schema.org/callSign>
pub const CALL_SIGN_PROPERTY_IRI_HTTP: &str = "http://schema.org/callSign";
/// <https://schema.org/callSign>
pub const CALL_SIGN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/callSign";
/// <https://schema.org/callSign>
pub const CALL_SIGN_PROPERTY_LABEL: &str = "callSign";
pub struct CallSignPropertyIri;
impl PartialEq<&str> for CallSignPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CALL_SIGN_PROPERTY_IRI_HTTP || *other == CALL_SIGN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CallSignPropertyIri> for &str {
	fn eq(&self, other: &CallSignPropertyIri) -> bool {
		*self == CALL_SIGN_PROPERTY_IRI_HTTP || *self == CALL_SIGN_PROPERTY_IRI_HTTPS
	}
}
pub struct CallSignPropertyIriOrLabel;
impl PartialEq<&str> for CallSignPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CallSignPropertyIri || *other == CALL_SIGN_PROPERTY_LABEL
	}
}
impl PartialEq<CallSignPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CallSignPropertyIriOrLabel) -> bool {
		*self == CallSignPropertyIri || *self == CALL_SIGN_PROPERTY_LABEL
	}
}
