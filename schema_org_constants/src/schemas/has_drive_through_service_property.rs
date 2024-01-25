/// <https://schema.org/hasDriveThroughService>
pub const HAS_DRIVE_THROUGH_SERVICE_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/hasDriveThroughService";
/// <https://schema.org/hasDriveThroughService>
pub const HAS_DRIVE_THROUGH_SERVICE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/hasDriveThroughService";
/// <https://schema.org/hasDriveThroughService>
pub const HAS_DRIVE_THROUGH_SERVICE_PROPERTY_LABEL: &str = "hasDriveThroughService";
pub struct HasDriveThroughServicePropertyIri;
impl PartialEq<&str> for HasDriveThroughServicePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_DRIVE_THROUGH_SERVICE_PROPERTY_IRI_HTTP
			|| *other == HAS_DRIVE_THROUGH_SERVICE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasDriveThroughServicePropertyIri> for &str {
	fn eq(&self, other: &HasDriveThroughServicePropertyIri) -> bool {
		*self == HAS_DRIVE_THROUGH_SERVICE_PROPERTY_IRI_HTTP
			|| *self == HAS_DRIVE_THROUGH_SERVICE_PROPERTY_IRI_HTTPS
	}
}
pub struct HasDriveThroughServicePropertyIriOrLabel;
impl PartialEq<&str> for HasDriveThroughServicePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasDriveThroughServicePropertyIri
			|| *other == HAS_DRIVE_THROUGH_SERVICE_PROPERTY_LABEL
	}
}
impl PartialEq<HasDriveThroughServicePropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasDriveThroughServicePropertyIriOrLabel) -> bool {
		*self == HasDriveThroughServicePropertyIri
			|| *self == HAS_DRIVE_THROUGH_SERVICE_PROPERTY_LABEL
	}
}
