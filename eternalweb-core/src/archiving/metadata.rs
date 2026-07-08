//! Metadata extraction and semantic tagging (Web 3.0)

use super::*;

/// Extract enhanced metadata from HTML content
pub async fn extract_semantic_metadata(html: &str, url: &str) -> Result<SnapshotMetadata> {
    let document = scraper::Html::parse_document(html);

    // Extract Schema.org structured data
    let schema_types = extract_schema_types(&document);

    // Extract Open Graph data
    let og_data = extract_opengraph(&document);

    // Extract Dublin Core metadata
    let dc_data = extract_dublin_core(&document);

    // Combine all metadata
    Ok(SnapshotMetadata {
        schema_types,
        title: og_data.title.or(dc_data.title),
        description: og_data.description.or(dc_data.description),
        creator: dc_data.creator,
        language: extract_language(&document),
        tags: extract_tags(&document),
        license: extract_license(&document),
        is_public: true,
    })
}

/// Extract Schema.org types from structured data
fn extract_schema_types(document: &scraper::Html) -> Vec<String> {
    let mut types = Vec::new();

    // Look for JSON-LD
    let script_selector = scraper::Selector::parse("script[type='application/ld+json']").unwrap();
    for script in document.select(&script_selector) {
        let text = script.text().collect::<String>();
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(schema_type) = json.get("@type").and_then(|t| t.as_str()) {
                types.push(schema_type.to_string());
            }
        }
    }

    // Look for microdata
    let itemtype_selector = scraper::Selector::parse("[itemtype]").unwrap();
    for el in document.select(&itemtype_selector) {
        if let Some(itemtype) = el.value().attr("itemtype") {
            if let Some(schema_type) = itemtype.split('/').last() {
                types.push(schema_type.to_string());
            }
        }
    }

    if types.is_empty() {
        types.push("WebPage".to_string());
    }

    types
}

/// Open Graph data
#[derive(Default)]
struct OpenGraphData {
    title: Option<String>,
    description: Option<String>,
}

/// Extract Open Graph metadata
fn extract_opengraph(document: &scraper::Html) -> OpenGraphData {
    let mut og = OpenGraphData::default();

    let og_selector = scraper::Selector::parse("meta[property^='og:']").unwrap();
    for meta in document.select(&og_selector) {
        if let (Some(property), Some(content)) = (meta.value().attr("property"), meta.value().attr("content")) {
            match property {
                "og:title" => og.title = Some(content.to_string()),
                "og:description" => og.description = Some(content.to_string()),
                _ => {}
            }
        }
    }

    og
}

/// Dublin Core data
#[derive(Default)]
struct DublinCoreData {
    title: Option<String>,
    description: Option<String>,
    creator: Option<String>,
}

/// Extract Dublin Core metadata
fn extract_dublin_core(document: &scraper::Html) -> DublinCoreData {
    let mut dc = DublinCoreData::default();

    let dc_selector = scraper::Selector::parse("meta[name^='DC.']").unwrap();
    for meta in document.select(&dc_selector) {
        if let (Some(name), Some(content)) = (meta.value().attr("name"), meta.value().attr("content")) {
            match name {
                "DC.title" => dc.title = Some(content.to_string()),
                "DC.description" => dc.description = Some(content.to_string()),
                "DC.creator" => dc.creator = Some(content.to_string()),
                _ => {}
            }
        }
    }

    // Fallback to standard meta tags
    if dc.title.is_none() {
        let title_selector = scraper::Selector::parse("title").unwrap();
        dc.title = document.select(&title_selector)
            .next()
            .map(|el| el.text().collect::<String>());
    }

    if dc.description.is_none() {
        let desc_selector = scraper::Selector::parse("meta[name='description']").unwrap();
        dc.description = document.select(&desc_selector)
            .next()
            .and_then(|el| el.value().attr("content"))
            .map(String::from);
    }

    dc
}

/// Extract language from HTML
fn extract_language(document: &scraper::Html) -> Option<String> {
    let html_selector = scraper::Selector::parse("html").unwrap();
    document.select(&html_selector)
        .next()
        .and_then(|el| el.value().attr("lang"))
        .map(String::from)
}

/// Extract tags/keywords
fn extract_tags(document: &scraper::Html) -> Vec<String> {
    let keywords_selector = scraper::Selector::parse("meta[name='keywords']").unwrap();
    document.select(&keywords_selector)
        .next()
        .and_then(|el| el.value().attr("content"))
        .map(|content| {
            content.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

/// Extract license information
fn extract_license(document: &scraper::Html) -> Option<String> {
    // Look for rel="license" links
    let license_selector = scraper::Selector::parse("a[rel='license'], link[rel='license']").unwrap();
    document.select(&license_selector)
        .next()
        .and_then(|el| el.value().attr("href"))
        .map(String::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_extraction() {
        let html = r#"<html><head><script type="application/ld+json">{"@type": "Article"}</script></head></html>"#;
        let document = scraper::Html::parse_document(html);
        let types = extract_schema_types(&document);
        assert!(types.contains(&"Article".to_string()));
    }
}
