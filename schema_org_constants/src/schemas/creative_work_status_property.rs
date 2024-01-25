/// <https://schema.org/creativeWorkStatus>
pub const CREATIVE_WORK_STATUS_PROPERTY_IRI_HTTP: &str = "http://schema.org/creativeWorkStatus";
/// <https://schema.org/creativeWorkStatus>
pub const CREATIVE_WORK_STATUS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/creativeWorkStatus";
/// <https://schema.org/creativeWorkStatus>
pub const CREATIVE_WORK_STATUS_PROPERTY_LABEL: &str = "creativeWorkStatus";
pub struct CreativeWorkStatusPropertyIri;
impl PartialEq<&str> for CreativeWorkStatusPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CREATIVE_WORK_STATUS_PROPERTY_IRI_HTTP
			|| *other == CREATIVE_WORK_STATUS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CreativeWorkStatusPropertyIri> for &str {
	fn eq(&self, other: &CreativeWorkStatusPropertyIri) -> bool {
		*self == CREATIVE_WORK_STATUS_PROPERTY_IRI_HTTP
			|| *self == CREATIVE_WORK_STATUS_PROPERTY_IRI_HTTPS
	}
}
pub struct CreativeWorkStatusPropertyIriOrLabel;
impl PartialEq<&str> for CreativeWorkStatusPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CreativeWorkStatusPropertyIri || *other == CREATIVE_WORK_STATUS_PROPERTY_LABEL
	}
}
impl PartialEq<CreativeWorkStatusPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CreativeWorkStatusPropertyIriOrLabel) -> bool {
		*self == CreativeWorkStatusPropertyIri || *self == CREATIVE_WORK_STATUS_PROPERTY_LABEL
	}
}
