# API schema boundary

`crates/api` owns HTTP response DTOs and request DTOs whose wire shape differs from the owning capability's behavior input. Known response bodies use concrete `ToSchema` types; `serde_json::Value` is reserved for genuinely open JSON fields.

Capability types may retain `utoipa` derives only when `crates/api` deliberately re-exports an identical thin input with a local type alias. This avoids two field-for-field DTOs while keeping route handlers dependent on their local `dto` module.

Current retained derives are:

- `iam::roles`: role CRUD and role assignment request bodies are identical to the behavior inputs used by `routes/roles/dto.rs`.
- `iam::users`: register, password, self-service, role-assignment, and list inputs are identical to the inputs used by `routes/users/dto.rs`. Update and reset inputs with path/body differences remain API-owned DTOs.
- `metadata::dictionaries`: the list query is identical to the input used by `routes/dictionaries/dto.rs`.
- `metadata::parameters`: list and mutation inputs are identical to the inputs used by `routes/parameters/dto.rs`.
- `file-storage::files`: the list query is identical to the input used by `routes/files/dto.rs`.
- `audit`: the audit list query is an intentional read-only exception reused by `routes/audit/events/dto.rs`.

Do not add a capability-side OpenAPI derive unless a local API type alias consumes it and this list is updated. Capability models and response views must not derive OpenAPI schemas merely for route documentation.
