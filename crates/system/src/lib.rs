pub mod api_registry;
pub mod authority;
pub mod dictionary;
pub mod errors;
pub mod logs;
pub mod menu;
pub mod params;
pub mod users;

#[cfg(test)]
mod tests {
    #[test]
    fn default_menu_payload_contains_dashboard_entry() {
        let menus = crate::menu::default_menus();

        assert!(!menus.is_empty());
        assert_eq!(menus[0].name, "dashboard");
        assert_eq!(menus[0].component, "view/dashboard/index.vue");
        assert_eq!(menus[0].meta.title, "Dashboard");
    }

    #[test]
    fn default_authorities_contains_super_admin() {
        let authorities = crate::authority::default_authorities();

        assert_eq!(authorities.len(), 1);
        assert_eq!(authorities[0].authority_id, 888);
        assert_eq!(authorities[0].authority_name, "Super Admin");
    }

    #[test]
    fn default_api_registry_descriptions_are_english() {
        let descriptions = crate::api_registry::default_api_descriptions();

        assert!(!descriptions.is_empty());
        assert!(
            descriptions
                .iter()
                .all(|description| description.is_ascii())
        );
        assert!(descriptions.contains(&"User login"));
    }
}
