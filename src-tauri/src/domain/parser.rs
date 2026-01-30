pub struct WorkshopMetadata {
    pub app_id: String,
    pub title: String,
    pub file_id: String,
}

pub fn parse_workshop_html(html: &str, file_id: &str) -> Result<WorkshopMetadata, String> {
    let app_id = extract_appid(html).ok_or_else(|| "Failed to extract AppID".to_string())?;
    let title = extract_title(html).ok_or_else(|| "Failed to extract Title".to_string())?;

    Ok(WorkshopMetadata {
        app_id,
        title,
        file_id: file_id.to_string(),
    })
}

fn extract_appid(html: &str) -> Option<String> {
    html.find("data-appid=\"")
        .and_then(|start| {
            let offset = start + 12;
            html.get(offset..)
                .and_then(|rest| rest.find('"').map(|end| rest[..end].to_string()))
        })
        .or_else(|| {
            html.find("/app/")
                .and_then(|start| {
                    let offset = start + 5;
                    html.get(offset..).map(|rest| {
                        rest.chars()
                            .take_while(|c| c.is_ascii_digit())
                            .collect::<String>()
                    })
                })
                .filter(|s| !s.is_empty())
        })
}

fn extract_title(html: &str) -> Option<String> {
    html.find("class=\"workshopItemTitle\"").and_then(|start| {
        html.get(start..).and_then(|rest| {
            rest.find('>').and_then(|gt_start| {
                let content_start = gt_start + 1;
                rest.get(content_start..).and_then(|content| {
                    content
                        .find("</div>")
                        .map(|end| content[..end].trim().to_string())
                })
            })
        })
    })
}
