// AST -> HTML thats all
// Might use Tera, haven't thought about it
// kairo-core/src/render.rs

pub fn render_page(title: &str, body: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <link rel="stylesheet" href="/style.css">
</head>
<body>
    <main>
        {body}
    </main>
</body>
</html>"#,
        title = title,
        body = body
    )
}

