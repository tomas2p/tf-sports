use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    // Minimal sitemap generator: collects static routes and writes sitemap.xml to assets/
    let domain = "https://tf-sports.example";
    let routes = vec!["/", "/events", "/sports", "/places"];

    let mut body = String::new();
    body.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    body.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
    let routes_len = routes.len();
    for r in &routes {
        body.push_str(" <url>\n");
        body.push_str(&format!(" <loc>{}{}</loc>\n", domain, r));
        body.push_str(" </url>\n");
    }
    body.push_str("</urlset>\n");

    let assets_dir = Path::new("assets");
    if !assets_dir.exists() {
        fs::create_dir_all(assets_dir)?;
    }

    let mut f = File::create(assets_dir.join("sitemap.xml"))?;
    f.write_all(body.as_bytes())?;

    println!("Wrote assets/sitemap.xml ({} entries)", routes_len);

    Ok(())
}
