use uuid::Uuid;

/// Distinct type so a user or blog id cannot be passed where a tenant is
/// expected. Every tenant-scoped query takes one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TenantId(pub Uuid);
