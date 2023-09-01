use axum::response::Html;

pub async fn handle_hello() -> &'static str {
    // sleep for 1.5 seconds to simulate a long running request
    tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
    "This is a message from the backend!"
}

pub fn gen_index_html() -> Html<String> {
    let asset_imports = if cfg!(debug_assertions) {
        String::from(
            r#"
          <script type="module" src="http://localhost:5173/@vite/client"></script>
          <script type="module" src="http://localhost:5173/src/index.tsx"></script>
          <link rel="stylesheet" href="http://localhost:5173/assets/index.css">
      "#,
        )
    } else {
        let js_file = find_asset(Asset::Js);
        let css_file = find_asset(Asset::Css);
        format!(
            r#"
          <script type="module" crossorigin src="/assets/{js_file}"></script>
          <link rel="stylesheet" href="/assets/{css_file}">
      "#,
            js_file = js_file
        )
    };

    Html(format!(
        r#"
  <!doctype html>
  <html lang="en">

  <head>
  <meta charset="UTF-8" />
  <!-- For old IEs -->
  <link rel="shortcut icon" href="favicon.ico" />

  <!-- For new browsers - multisize ico  -->
  <link rel="icon" type="image/x-icon" sizes="16x16 32x32" href="favicon.ico">

  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Solid + Rust</title>
  {asset_imports}
  </head>

  <body>
  <div id="root"></div>

  </body>

  </html>
  "#,
        asset_imports = asset_imports
    ))
}

enum Asset {
    Css,
    Js,
}

fn find_asset(asset: Asset) -> String {
    let file_ending = match asset {
        Asset::Css => ".css",
        Asset::Js => ".js",
    };
    std::fs::read_dir("frontend/dist/assets")
        .unwrap()
        .find_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let path = path.to_str().unwrap();
                if path.ends_with(file_ending) {
                    path.split('/');
                    return Some(path.split('/').last().unwrap().to_string());
                }
            };
            None
        })
        .unwrap()
}
