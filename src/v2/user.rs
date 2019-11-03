//! Interface for users in JIRA

// ============================================================================
// Use
// ============================================================================
use crate::client::Client;
use crate::v2::{
    application_role::ApplicationRole, group::Group, item::Item, pagination::Pagination,
};
use crate::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::BTreeMap;
use std::collections::HashMap;

// ============================================================================
// Public Enums
// ============================================================================
pub enum Expand {
    Groups,
    ApplicationRoles,
}

impl Expand {
    pub fn to_string(&self) -> &str {
        match &self {
            Expand::Groups => "groups",
            Expand::ApplicationRoles => "applicationRoles",
        }
    }
}

// ============================================================================
// Public Structures
// ============================================================================
pub struct UserOptions {
    /// Include inactive users in requests.
    include_inactive: bool,

    /// Include active users in requests.
    include_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// Returns true if the user is active in Jira
    #[serde(default)]
    pub active: bool,

    /// Returns avatar urls for the user
    #[serde(rename = "avatarUrls", default)]
    pub avatar_urls: BTreeMap<String, String>,

    /// Returns the display name of the user
    #[serde(rename = "displayName", default)]
    pub display_name: String,

    /// The email address associated with the account
    #[serde(rename = "emailAddress", default)]
    pub email_address: String,

    /// The id key associated with the user
    #[serde(default)]
    pub key: String,

    /// The username of the user
    #[serde(default)]
    pub name: String,

    /// A link to the user object
    #[serde(rename = "self", default)]
    pub self_link: String,

    /// The timezone set for the user account
    #[serde(rename = "timeZone", default)]
    pub timezone: String,

    /// List of groups the user belongs to
    #[serde()]
    pub groups: Option<Item>,

    /// List of application roles the user has
    #[serde(rename = "applicationRoles")]
    pub application_roles: Option<Item>,
}

impl User {
    pub fn search<S>(
        c: &Client,
        search: S,
        opts: Option<UserOptions>,
        page: Option<Pagination>,
    ) -> Result<Vec<User>>
    where
        S: Into<String>,
    {
        let mut query: HashMap<String, String> = HashMap::new();
        query.insert("username".to_string(), search.into());

        if let Some(o) = opts {
            query.insert(
                "includeInactive".to_string(),
                o.include_inactive.to_string(),
            );

            query.insert("includeActive".to_string(), o.include_active.to_string());
        }

        if let Some(p) = page {
            query.insert("startAt".to_string(), p.start_at.to_string());
            query.insert("maxResults".to_string(), p.max_results.to_string());
        }

        c.get("api", "2", "user/search", Some(query), None)
    }

    /// Fetches a user by username
    pub fn from_username<U>(c: &Client, username: U, expand: &[Expand]) -> Result<User>
    where
        U: Into<String>,
    {
        let mut query: HashMap<String, String> = HashMap::new();
        query.insert("username".to_string(), username.into());
        query.extend(expand_to_hashmap(expand));

        c.get("api", "2", "user", Some(query), None)
    }

    /// Fetches a user by key
    pub fn from_key<K>(c: &Client, key: K, expand: &[Expand]) -> Result<User>
    where
        K: Into<String>,
    {
        let mut query: HashMap<String, String> = HashMap::new();
        query.insert("key".to_string(), key.into());
        query.extend(expand_to_hashmap(expand));

        c.get("api", "2", "user", Some(query), None)
    }

    pub fn groups(&self) -> Vec<Group> {
        if let Some(i) = &self.groups {
            serde_json::value::from_value(i.items.clone()).unwrap()
        } else {
            Vec::new()
        }
    }

    pub fn application_roles(&self) -> Vec<ApplicationRole> {
        if let Some(i) = &self.application_roles {
            serde_json::value::from_value(i.items.clone()).unwrap()
        } else {
            Vec::new()
        }
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for User {
    // This trait requires fmt with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

// ============================================================================
// Private
// ============================================================================
fn expand_to_hashmap(e: &[Expand]) -> HashMap<String, String> {
    let mut res: HashMap<String, String> = HashMap::new();
    let mut value = e.iter().fold(String::from(""), |acc, e| {
        format!("{}{},", acc, e.to_string())
    });

    value.pop();

    res.insert("expand".to_string(), value);
    res
}

// ============================================================================
// Tests
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_to_hashmap() {
        let e = vec![Expand::ApplicationRoles, Expand::Groups];
        let h = expand_to_hashmap(&e);

        assert!(h.get("expand").unwrap().contains("groups"));
        assert!(h.get("expand").unwrap().contains("applicationRoles"));
    }
}
