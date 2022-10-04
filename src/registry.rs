use std::fmt;

use crate::github::Content;
use eyre::{eyre, Context, Result};
use http::{Method, StatusCode};
use serde::de::DeserializeOwned;

pub use crate::{assets::*, chain::*, paths::*};

const VERSION: &str = env!("CARGO_PKG_VERSION");
pub struct Repo {
    pub git_ref: Ref,
    pub url: String,
    pub raw_file_url: String,
}

impl Default for Repo {
    fn default() -> Self {
        Self {
            git_ref: Ref::SHA("1ec726b7308a71ce0cb02916b1929979c6f2e39d".to_string()),
            url: "https://api.github.com/repos/cosmos/chain-registry/contents".to_string(),
            raw_file_url: "https://raw.githubusercontent.com/cosmos/chain-registry".to_string(),
        }
    }
}

pub enum Ref {
    /// Use the latest commit
    Latest,
    /// Use a specific git sha
    SHA(String),
}
impl fmt::Display for Ref {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ref::Latest => write!(f, "master"),
            Ref::SHA(sha) => write!(f, "{sha}"),
        }
    }
}

pub struct Registry {
    pub repo: Repo,
}

impl Registry {
    /// Returns a [`Registry`] optionally configuring a [`Repo`]
    ///
    /// # Arguments
    ///
    /// * `repo` - provide an optional [`Repo`] config, overriding defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// use chain_registry::registry::{Registry, Repo, Ref};
    ///
    /// // Use repo defaults:
    /// let reg = Registry::new(None);
    ///
    /// // Use custom repo settings:
    /// let repo = Repo{
    ///     git_ref: Ref::SHA("1ec726b7308a71ce0cb02916b1929979c6f2e39d".to_string()),
    ///     ..Default::default()
    /// };
    /// let reg = Registry::new(Some(repo));
    /// ```
    pub fn new(repo: Option<Repo>) -> Self {
        let repo = repo.unwrap_or_default();
        Self { repo }
    }

    /// Gets a list of chain names from the registry
    pub async fn list_chains(&self) -> Result<Vec<String>> {
        let url = format!("{}?ref={}", &self.repo.url, &self.repo.git_ref,);
        let json: String = get(url).await?;
        let contents: Vec<Content> = serde_json::from_str(json.as_str())?;

        Ok(contents
            .iter()
            .filter(|c| c.type_field == "dir" && !c.name.starts_with('_') && c.name != ".github")
            .map(|c| c.clone().name)
            .collect())
    }

    /// Gets a list of path names from the registry in the form <chain_a>-<chain_b>
    pub async fn list_paths(&self) -> Result<Vec<String>> {
        let url = format!("{}/_IBC?ref={}", &self.repo.url, &self.repo.git_ref,);
        let json: String = get(url).await?;
        let contents: Vec<Content> = serde_json::from_str(json.as_str())?;

        Ok(contents
            .iter()
            .filter(|c| {
                c.type_field == "file" && !c.name.starts_with('_') && c.name.ends_with(".json")
            })
            .map(|c| c.name[..c.name.len() - ".json".len()].to_string())
            .collect())
    }

    /// Retrieves the deserialized `assets.json` for a given chain. The result will contain
    /// `None` if the there is no `assets.json` present.
    ///
    /// # Arguments
    ///
    /// * `name` - The chain name. Must match the name of the chain's folder in the root directory of the
    /// [chain registry](https://github.com/cosmos/chain-registry).
    pub async fn get_assets(&self, name: &str) -> Result<Option<AssetList>> {
        let path = format!("{}/assetlist.json", name);
        let data = get_file_content(&path, &self.repo).await?;

        Ok(parse_json(data).await)
    }

    /// Retrieves the deserialized `chain.json` for a given chain. The result will contain
    /// `None` if the there is no `chain.json` present.
    ///
    /// # Arguments
    ///
    /// * `name` - The chain name. Must match the name of the chain's folder in the root directory of the
    /// [chain registry](https://github.com/cosmos/chain-registry).
    pub async fn get_chain(&self, name: &str) -> Result<Option<ChainInfo>> {
        let path = format!("{}/chain.json", name);
        let data = get_file_content(&path, &self.repo).await?;

        Ok(parse_json(data).await)
    }

    /// Retrieves the deserialized IBC path json for a given pair of chains. The result will contain
    /// `None` if the there is no path present.
    ///
    /// # Arguments
    ///
    /// * `name` - The chain name. Must match the name of the chain's folder in the root directory of the
    /// [chain registry](https://github.com/cosmos/chain-registry).
    pub async fn get_path(&self, chain_a: &str, chain_b: &str) -> Result<Option<IBCPath>> {
        // path names order the chain names alphabetically
        let path = format!(
            "_IBC/{}-{}.json",
            chain_a.min(chain_b),
            chain_a.max(chain_b)
        );
        let data = get_file_content(&path, &self.repo).await?;

        Ok(parse_json(data).await)
    }
}

async fn get(url: String) -> Result<String> {
    let client = reqwest::Client::new();
    let req = client
        .request(Method::GET, url)
        .header("User-Agent", format!("ocular/{}", VERSION))
        .build()?;
    Ok(client.execute(req).await?.text().await?)
}

async fn get_file_content(path: &str, repo: &Repo) -> Result<String> {
    let url = format!("{}/{}/{}", &repo.raw_file_url, &repo.git_ref, path);
    let response = reqwest::get(url).await?; //.text().await?

    if response.status() == StatusCode::NOT_FOUND {
        return Err(eyre!("path {} not found", path));
    }

    response
        .text()
        .await
        .wrap_err("error getting remote file content")
}

async fn parse_json<T>(data: String) -> Option<T>
where
    T: core::fmt::Debug + DeserializeOwned,
{
    let result = serde_json::from_str(&data);
    println!("{:?}", result);

    result.ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use assay::assay;

    #[assay]
    async fn gets_content_from_registry() {
        let repo = Repo::default();
        let result = get_file_content("cosmoshub/chain.json", &repo).await;
        result.unwrap();

        // custom ref works:
        let repo = Repo {
            git_ref: Ref::Latest,
            ..Default::default()
        };
        let result = get_file_content("cosmoshub/chain.json", &repo).await;
        result.unwrap();
    }

    #[assay]
    async fn parses_chain_info() {
        let repo = Repo::default();
        let result = get_file_content("cosmoshub/chain.json", &repo)
            .await
            .unwrap();
        let result = parse_json::<ChainInfo>(result).await;
        result.unwrap();

        // custom ref works:
        let repo = Repo {
            git_ref: Ref::SHA("8d84b83cbead0c61de666b709a036cc829426eef".to_string()),
            ..Default::default()
        };
        let result = get_file_content("cosmoshub/chain.json", &repo)
            .await
            .unwrap();
        let result = parse_json::<ChainInfo>(result).await;
        result.unwrap();
    }

    #[assay]
    async fn gets_chain() {
        let registry = Registry::new(None);
        let result = registry.get_chain("cosmoshub").await;
        result.unwrap();

        // custom ref works:
        let registry = Registry::new(Some(Repo {
            git_ref: Ref::SHA("cfd65eaa48ec6db79cd4e8dea9692f782f08de29".to_string()),
            ..Default::default()
        }));
        let result = registry.get_chain("cosmoshub").await;
        result.unwrap();
    }

    #[assay]
    async fn lists_chains() {
        let registry = Registry::new(None);
        registry.list_chains().await.unwrap();
    }

    #[assay]
    async fn lists_paths() {
        let registry = Registry::new(None);
        let paths = registry.list_paths().await.unwrap();
        assert!(paths.len() > 0);
        paths
            .iter()
            .for_each(|path| assert!(!path.ends_with(".json")))
    }

    #[assay]
    async fn gets_path_in_order() {
        let chain_a = "cosmoshub";
        let chain_b = "osmosis";
        let registry = Registry::new(None);
        let result = registry.get_path(chain_a, chain_b).await.unwrap().unwrap();
        assert_eq!(result.chain_1.chain_name, "cosmoshub");
        assert_eq!(result.chain_2.chain_name, "osmosis");
    }

    #[assay]
    async fn gets_path_out_of_order() {
        let chain_a = "cosmoshub";
        let chain_b = "osmosis";
        let registry = Registry::new(None);
        let result = registry.get_path(chain_b, chain_a).await.unwrap().unwrap();
        assert_eq!(result.chain_1.chain_name, "cosmoshub");
        assert_eq!(result.chain_2.chain_name, "osmosis");
    }

    #[assay]
    async fn get_path_not_present_errors() {
        let chain_a = "fake";
        let chain_b = "osmosis";
        let registry = Registry::new(None);
        let result = registry.get_path(chain_b, chain_a).await;
        assert!(result.is_err())
    }
}
