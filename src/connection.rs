use ureq::Error;

pub fn get_prompt(name: String) -> Result<String, Error> {
    let content = ureq::get(
        format!(
            "https://raw.githubusercontent.com/AWeirdDev/prompts/main/prompts/{}.md",
            name
        )
        .as_str(),
    )
    .set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36 OPR/112.0.0.0")
    .call()?
    .into_string()?;

    Ok(content)
}
