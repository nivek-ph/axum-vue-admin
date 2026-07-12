pub mod api_permissions;
pub mod apis;
pub mod authority;
pub mod authorization;
pub mod data_scope;
pub mod departments;
pub mod permissions;
pub mod roles;
pub mod users;

#[cfg(test)]
mod tests {
    #[test]
    fn default_authorities_contains_super_admin() {
        let authorities = crate::authority::default_authorities();

        assert_eq!(authorities.len(), 1);
        assert_eq!(authorities[0].authority_id, 1);
        assert_eq!(authorities[0].authority_name, "Super Admin");
    }
}
