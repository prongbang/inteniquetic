use crate::types::launchpad::ProjectLink;
use std::collections::HashSet;

pub fn make_title(url: &str) -> String {
    match url::Url::parse(url) {
        Ok(parsed) => {
            let host = parsed.host_str().unwrap_or_default();
            if host.contains("crates.io") {
                return parsed
                    .path_segments()
                    .and_then(|mut segments| segments.next_back())
                    .unwrap_or(url)
                    .to_string();
            }

            if host.contains("github.com") {
                let segments = parsed
                    .path_segments()
                    .map(|seg| seg.collect::<Vec<_>>())
                    .unwrap_or_default();
                if segments.len() >= 2 {
                    return segments[1].to_string();
                }
            }

            host.to_string()
        }
        Err(_) => url.to_string(),
    }
}

pub fn host_type(url: &str) -> (&'static str, &'static str) {
    match url::Url::parse(url)
        .ok()
        .and_then(|parsed| parsed.host_str().map(str::to_string))
    {
        Some(host) if host.contains("github.com") => ("github.com", "GitHub"),
        Some(host) if host.contains("crates.io") => ("crates.io", "Crate"),
        Some(_) => ("external", "Link"),
        None => ("unknown", "Link"),
    }
}

pub fn filter_links(category: &str, links: &[ProjectLink], query: &str) -> Vec<ProjectLink> {
    let deduped_links = dedupe_links(links);
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return deduped_links;
    }

    deduped_links
        .iter()
        .filter(|link| {
            let title = make_title(&link.url).to_lowercase();
            title.contains(&q)
                || link.url.to_lowercase().contains(&q)
                || category.to_lowercase().contains(&q)
        })
        .cloned()
        .collect()
}

pub fn dedupe_links(links: &[ProjectLink]) -> Vec<ProjectLink> {
    let mut seen = HashSet::new();
    links
        .iter()
        .filter(|link| seen.insert(link.url.clone()))
        .cloned()
        .collect()
}
