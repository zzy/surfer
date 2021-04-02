// pub async fn web_base_uri() -> String {
//     // let web_prot = CFG.get("WEB_PROT").unwrap();
//     let web_addr = CFG.get("WEB_ADDR").unwrap();
//     let web_port = CFG.get("WEB_PORT").unwrap();

//     // format!("{}://{}:{}", web_prot, web_addr, web_port)
//     format!("//{}:{}", web_addr, web_port)
// }

// Generate friendly slug from the given string
pub async fn slugify(str: &str) -> String {
    use deunicode::deunicode_with_tofu;

    let slug = deunicode_with_tofu(str.trim(), "-")
        .to_lowercase()
        .replace(" ", "-")
        .replace("[", "-")
        .replace("]", "-")
        .replace("\"", "-")
        .replace("/", "-");

    slug
}
