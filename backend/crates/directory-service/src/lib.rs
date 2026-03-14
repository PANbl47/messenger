mod projection;

use persistence::{InMemoryStore, StoredAccount};

pub use projection::{DirectoryEntry, DisplayNameResult};

#[derive(Debug, Clone)]
pub struct DirectoryService {
    store: InMemoryStore,
}

impl DirectoryService {
    pub fn new(store: InMemoryStore) -> Self {
        Self { store }
    }

    pub async fn search_by_username(&self, username: &str) -> Option<DirectoryEntry> {
        self.store.account_by_username(username).map(map_directory_entry)
    }

    pub async fn search_by_display_name(&self, query: &str) -> Vec<DisplayNameResult> {
        let needle = query.trim().to_ascii_lowercase();
        let mut results: Vec<(u8, DisplayNameResult)> = self
            .store
            .all_accounts()
            .into_iter()
            .filter_map(|account| rank_display_name(&account, &needle))
            .collect();

        results.sort_by(|left, right| {
            left.0
                .cmp(&right.0)
                .then_with(|| left.1.display_name.cmp(&right.1.display_name))
                .then_with(|| left.1.username.cmp(&right.1.username))
        });

        results.into_iter().map(|(_, entry)| entry).collect()
    }

    pub async fn search_by_phone(&self, phone: &str) -> Option<DirectoryEntry> {
        self.store
            .account_by_phone(phone)
            .filter(|account| account.phone_discoverable)
            .map(map_directory_entry)
    }
}

fn map_directory_entry(account: StoredAccount) -> DirectoryEntry {
    DirectoryEntry {
        account_id: account.account_id,
        username: account.username,
        display_name: account.display_name,
    }
}

fn rank_display_name(account: &StoredAccount, query: &str) -> Option<(u8, DisplayNameResult)> {
    let haystack = account.display_name.to_ascii_lowercase();
    let score = if haystack == query {
        0
    } else if haystack.starts_with(query) {
        1
    } else if haystack.contains(query) {
        2
    } else {
        return None;
    };

    Some((
        score,
        DisplayNameResult {
            account_id: account.account_id.clone(),
            username: account.username.clone(),
            display_name: account.display_name.clone(),
            disambiguation: format!("@{}", account.username),
        },
    ))
}
