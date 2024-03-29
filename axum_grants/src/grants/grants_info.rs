#[derive(Debug)]
pub(crate) struct GrantInfo {
    pub(crate) perms: Vec<String>,
    pub(crate) operator: i8,
}

impl GrantInfo {
    // parse GrantInfo from attribute string
    pub(crate) fn parse(attr: &str) -> Option<GrantInfo> {
        let attr_temp = attr.trim().replace("\"", "");
        if attr_temp.is_empty() {
            return None;
        }

        // any or all operator symbol
        let (operator, perms) = if (attr_temp.starts_with("any(") || attr_temp.starts_with("all("))
            && attr_temp.ends_with(")")
        {
            let operator = if attr_temp.starts_with("any(") { 1 } else { 2 };
            let attr_temp = &attr_temp[4..attr_temp.len() - 1];
            let perms = attr_temp
                .split(",")
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            (operator, perms)
        }
        // equal operator
        else {
            (0, vec![attr_temp.to_string()])
        };

        // only one perm, the relations is equal
        if perms.len() == 1 {
            return Some(GrantInfo { perms, operator: 0 });
        }

        Some(GrantInfo { perms, operator })
    }
}
