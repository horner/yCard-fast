use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow;

/// Internationalization alias data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasPack {
    pub version: String,
    pub locales: HashMap<String, LocaleData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocaleData {
    #[serde(rename = "keyAliases")]
    pub key_aliases: HashMap<String, String>,
    #[serde(rename = "typeAliases")]
    pub type_aliases: HashMap<String, String>,
    pub countries: Option<HashMap<String, String>>,
    pub honorifics: Option<Vec<String>>,
}

/// Global alias manager
#[derive(Clone)]
pub struct AliasManager {
    packs: Vec<AliasPack>,
    default_locale: String,
}

impl AliasManager {
    pub fn new() -> Self {
        let mut manager = Self {
            packs: Vec::new(),
            default_locale: "en".to_string(),
        };
        
        // Load baked-in fallback
        manager.load_fallback_pack();
        manager
    }

    fn load_fallback_pack(&mut self) {
        let fallback_json = include_str!("../data/aliases.fallback.json");
        if let Ok(pack) = serde_json::from_str::<AliasPack>(fallback_json) {
            self.packs.push(pack);
        }
    }

    pub fn load_pack(&mut self, pack_json: &str) -> anyhow::Result<()> {
        let pack: AliasPack = serde_json::from_str(pack_json)?;
        self.packs.push(pack);
        Ok(())
    }

    pub fn load_pack_bytes(&mut self, bytes: &[u8]) -> anyhow::Result<()> {
        let pack: AliasPack = serde_json::from_slice(bytes)?;
        self.packs.push(pack);
        Ok(())
    }

    pub fn clear_packs(&mut self) {
        self.packs.clear();
        self.load_fallback_pack();
    }

    pub fn set_default_locale(&mut self, locale: &str) {
        self.default_locale = locale.to_string();
    }

    /// Resolve a key alias using BCP-47 fallback chain
    pub fn resolve_key_alias(&self, key: &str, locale: Option<&str>) -> Option<String> {
        let locale = locale.unwrap_or(&self.default_locale);
        let locales = self.build_locale_chain(locale);

        for pack in self.packs.iter().rev() {
            for loc in &locales {
                if let Some(locale_data) = pack.locales.get(loc) {
                    if let Some(alias) = locale_data.key_aliases.get(&self.normalize_key(key)) {
                        return Some(alias.clone());
                    }
                }
            }
        }
        None
    }

    /// Resolve a type alias using BCP-47 fallback chain
    pub fn resolve_type_alias(&self, type_name: &str, locale: Option<&str>) -> Option<String> {
        let locale = locale.unwrap_or(&self.default_locale);
        let locales = self.build_locale_chain(locale);

        for pack in self.packs.iter().rev() {
            for loc in &locales {
                if let Some(locale_data) = pack.locales.get(loc) {
                    if let Some(alias) = locale_data.type_aliases.get(&self.normalize_key(type_name)) {
                        return Some(alias.clone());
                    }
                }
            }
        }
        None
    }

    /// Build BCP-47 fallback chain: fr-CA -> fr -> root
    fn build_locale_chain(&self, locale: &str) -> Vec<String> {
        let mut chain = vec![locale.to_string()];
        
        if let Some(lang) = locale.split('-').next() {
            if lang != locale {
                chain.push(lang.to_string());
            }
        }
        
        if locale != "root" {
            chain.push("root".to_string());
        }
        
        chain
    }

    /// Normalize key for case/diacritic insensitive matching
    fn normalize_key(&self, key: &str) -> String {
        use unicode_normalization::UnicodeNormalization;
        
        // Simple normalization: decompose and remove common diacritics
        key.nfd()
            .filter(|c| {
                let c_u32 = *c as u32;
                // Filter out combining diacritical marks
                !(0x0300..=0x036F).contains(&c_u32)
            })
            .collect::<String>()
            .to_lowercase()
    }
}

impl Default for AliasManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_chain() {
        let manager = AliasManager::new();
        let chain = manager.build_locale_chain("fr-CA");
        assert_eq!(chain, vec!["fr-CA", "fr", "root"]);
    }

    #[test]
    fn test_key_normalization() {
        let manager = AliasManager::new();
        assert_eq!(manager.normalize_key("Téléphone"), "telephone");
        assert_eq!(manager.normalize_key("MÓVIL"), "movil");
    }
}