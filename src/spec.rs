//! Immutable happy-wakey identity and provider-operation contract.

use ore_mcp_org_server::OrgSpec;

const DEPENDENCIES: &[&str] = &[
    "ORESoftware/mcp-rust-libs",
    "ores-otel/ores-mcp-server-core-libs.rs",
    "shared-auth/shared-auth-clients",
    "shared-auth/shared-auth-interfaces",
    "shared-auth/shared-auth-lib",
    "zed-pkg/zed-cli",
];

/// Operations inherited from the pinned shared provider implementation.
pub const PROVIDER_OPERATIONS: &[(&str, &[&str])] = &[
    ("github", &["read_organization", "read_latest_workflow_run"]),
    ("aws", &["read_caller_identity", "read_eks_clusters"]),
    ("gcp", &["read_project", "read_enabled_services"]),
    ("supabase", &["read_auth_settings", "read_data_api_schema"]),
    ("neon", &["read_projects", "read_project_branches"]),
    ("cloudflare", &["read_zone", "read_dns_records"]),
    ("k8s_cluster", &["read_deployments", "read_pods"]),
    (
        "nats",
        &["read_service_snapshot", "read_dependency_snapshot"],
    ),
];

/// Resolve the exact read-only operation allowlist for one provider.
/// Unknown or case-variant providers have no implicit capabilities.
#[must_use]
pub fn allowed_provider_operations(provider: &str) -> Option<&'static [&'static str]> {
    PROVIDER_OPERATIONS
        .iter()
        .find(|(name, _)| *name == provider)
        .map(|(_, operations)| *operations)
}

/// Returns the exact organization, repository, service, and Zed dependency identity.
#[must_use]
pub fn org_spec() -> OrgSpec {
    debug_assert_eq!(PROVIDER_OPERATIONS.len(), 8);
    debug_assert!(allowed_provider_operations("github").is_some());
    OrgSpec {
        organization: "happy-wakey",
        repository: "happy-wakey/happy-wakey-mcp-server.rs",
        service_name: "happy-wakey-mcp-server",
        package_name: "happy-wakey-mcp-server",
        dependencies: DEPENDENCIES,
        version: env!("CARGO_PKG_VERSION"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope_and_provider_contract_are_exact() {
        let spec = org_spec();
        assert_eq!(spec.organization, "happy-wakey");
        assert_eq!(spec.repository, "happy-wakey/happy-wakey-mcp-server.rs");
        assert_eq!(PROVIDER_OPERATIONS.len(), 8);
        assert!(PROVIDER_OPERATIONS.iter().all(|(provider, operations)| {
            !provider.contains('*')
                && operations.len() == 2
                && operations
                    .iter()
                    .all(|operation| operation.starts_with("read_"))
        }));
    }
    #[test]
    fn provider_lookup_is_exact_and_never_escalates_unknown_names() {
        for (provider, expected) in PROVIDER_OPERATIONS {
            assert_eq!(allowed_provider_operations(provider), Some(*expected));
        }
        assert_eq!(allowed_provider_operations("GitHub"), None);
        assert_eq!(allowed_provider_operations("github.read_organization"), None);
        assert_eq!(allowed_provider_operations(""), None);
    }

}
