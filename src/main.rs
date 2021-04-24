use goose::goose::GooseResponse;
use goose::prelude::*;

use regex::Regex;

// Example:
//   cargo run --release -- -H https://tag1.com/ -v -u100

fn main() -> Result<(), GooseError> {
    let _goose_metrics = GooseAttack::initialize()?
        // 95% of the users load a single high-interest blog.
        .register_taskset(
            taskset!("Blog")
                .set_weight(95)?
                .set_wait_time(0, 3)?
                .register_task(task!(blog).set_name("decoupling apps")),
        )
        // 4% of the users load the front page to learn more about Tag1.
        .register_taskset(
            taskset!("Front page")
                .set_weight(4)?
                .set_wait_time(1, 3)?
                .register_task(task!(front).set_name("/")),
        )
        // 1% of the users load the blog listing page to look for more content.
        .register_taskset(
            taskset!("Blog listing page")
                .set_weight(1)?
                .set_wait_time(1, 3)?
                .register_task(task!(listing).set_name("/blog")),
        )
        .execute()?
        .print();

    Ok(())
}

// Load a blog page.
async fn blog(user: &GooseUser) -> GooseTaskResult {
    let goose = user.get("/blog/deep-dive-decoupling-applications-overview-decoupled-applications-systems-part-1").await?;
    validate_and_load_static_assets(user, goose, "A Deep Dive on Decoupling Applications").await?;

    Ok(())
}

// Load the front page.
async fn front(user: &GooseUser) -> GooseTaskResult {
    let goose = user.get("/").await?;
    validate_and_load_static_assets(user, goose, "Black Lives Matter | Tag1 Consulting").await?;

    Ok(())
}

// Load the blog listing page.
async fn listing(user: &GooseUser) -> GooseTaskResult {
    let goose = user.get("/blog").await?;
    validate_and_load_static_assets(user, goose, "Blogs | Tag1 Consulting").await?;

    Ok(())
}

/// Finds all local static elements on the page and loads them asynchronously.
/// This default profile only has local assets, so we can use simple patterns.
pub async fn load_static_elements(user: &GooseUser, html: &str) {
    let mut urls = Vec::new();

    // Use a regular expression to find all src=<foo> in the HTML, where foo
    // is the relative path to an image or js assets.
    let image = Regex::new(r#"src="(/.*?)""#).unwrap();
    for url in image.captures_iter(&html) {
        if url[1].starts_with("/sites")
            || url[1].starts_with("/core")
            || url[1].starts_with("/themes")
        {
            urls.push(url[1].to_string());
        }
    }

    // Use a regular expression to find all href=<foo> in the HTML, where foo
    // is the relative path to a css assets.
    let css = Regex::new(r#"href="(/.*?)""#).unwrap();
    for url in css.captures_iter(&html) {
        if url[1].starts_with("/sites") || url[1].starts_with("/themes") {
            urls.push(url[1].to_string());
        }
    }

    // Uncomment to debug-print all static assets being loaded:
    //println!("static assets: {:#?}", urls);

    // Load all the static assets found on the page.
    for asset in &urls {
        let _ = user.get_named(asset, "local static asset").await;
    }
}

/// A valid title on this website starts with "<title>foo", where "foo" is the expected
/// title text. Returns true if the expected title is set, otherwise returns false.
pub fn valid_title(html: &str, title: &str) -> bool {
    html.contains(&("<title>".to_string() + title))
}

/// Validate the HTML response, confirming the expected title was returned, then load
/// all static assets found on the page.
pub async fn validate_and_load_static_assets(
    user: &GooseUser,
    mut goose: GooseResponse,
    title: &str,
) -> GooseTaskResult {
    match goose.response {
        Ok(response) => {
            // Copy the headers so we have them for logging if there are errors.
            let headers = &response.headers().clone();
            match response.text().await {
                Ok(html) => {
                    if !valid_title(&html, &title) {
                        return user.set_failure(
                            &format!("{}: title not found: {}", goose.request.url, title),
                            &mut goose.request,
                            Some(&headers),
                            Some(&html),
                        );
                    }

                    load_static_elements(user, &html).await;
                }
                Err(e) => {
                    return user.set_failure(
                        &format!("{}: failed to parse page: {}", goose.request.url, e),
                        &mut goose.request,
                        Some(&headers),
                        None,
                    );
                }
            }
        }
        Err(e) => {
            return user.set_failure(
                &format!("{}: no response from server: {}", goose.request.url, e),
                &mut goose.request,
                None,
                None,
            );
        }
    }

    Ok(())
}
