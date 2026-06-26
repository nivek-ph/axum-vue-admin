#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataScope {
    All,
    Dept,
    DeptAndChildren,
    SelfOnly,
    CustomDepts,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataScopeFilter {
    All,
    DeptIds(Vec<i64>),
    Owner(i64),
}

pub fn merge_scopes(scopes: &[DataScope]) -> DataScope {
    if scopes.contains(&DataScope::All) {
        return DataScope::All;
    }
    if scopes.contains(&DataScope::DeptAndChildren) {
        return DataScope::DeptAndChildren;
    }
    if scopes.contains(&DataScope::CustomDepts) {
        return DataScope::CustomDepts;
    }
    if scopes.contains(&DataScope::Dept) {
        return DataScope::Dept;
    }
    DataScope::SelfOnly
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_scopes_uses_broadest_scope() {
        let scopes = vec![DataScope::SelfOnly, DataScope::Dept, DataScope::All];
        assert_eq!(merge_scopes(&scopes), DataScope::All);
    }

    #[test]
    fn merge_scopes_prefers_dept_children_over_custom_and_dept() {
        let scopes = vec![
            DataScope::Dept,
            DataScope::CustomDepts,
            DataScope::DeptAndChildren,
        ];
        assert_eq!(merge_scopes(&scopes), DataScope::DeptAndChildren);
    }

    #[test]
    fn merge_scopes_defaults_to_self_for_empty_input() {
        assert_eq!(merge_scopes(&[]), DataScope::SelfOnly);
    }
}
