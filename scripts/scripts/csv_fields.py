SPECIAL_CASE_REPORT_ITEM_FIELDS = (
    "spec_export_customers",
    "spec_as_is_origin_but_no_route",
    "spec_as_set_contains_origin_but_no_route",
    "spec_import_customer",
    "spec_import_from_neighbor",
    "spec_uphill",
    "spec_uphill_tier1",
    "spec_tier1_pair",
    "spec_peer_only_provider_policies",
    "spec_customer_only_provider_policies",
    "spec_other_only_provider_policies",
)

UNRECORDED_CASE_REPORT_ITEM_FIELDS = (
    "unrec_import_empty",
    "unrec_export_empty",
    "unrec_aut_num",
    "unrec_as_set_route",
    "unrec_some_as_set_route",
    "unrec_as_set",
    "unrec_some_as_set",
    "unrec_as_routes",
    "unrec_route_set",
    "unrec_peering_set",
    "unrec_filter_set",
)

MODIFIED_SPECIAL_CASE_FIELDS = (
    "spec_export_customers",
    "spec_import_customer",
    "spec_as_.*origin.*",
    "spec_.*_only_provider_policies",
    # "spec_tier1_pair",
    "spec_uphill_tier1",
    "spec_uphill",
)

MODIFIED_SPECIAL_CASE_LABELS = (
    "Export Self",
    "Import Customer",
    "Missing Route",
    "Only Provider",
    # "Tier-1 Peering", # Invisible
    "Uphill Tier-1",
    "Uphill",
)
