use goose::prelude::*;

// Example:
//   cargo run -- -H https://tag1.com/ -v -u1

// Load a blog page
async fn tag1blog(user: &GooseUser) -> GooseTaskResult {
  let _goose = user.get("blog/deep-dive-decoupling-applications-overview-decoupled-applications-systems-part-1").await?;

  // @TODO: load images, css and js from page
  // @TODO: rarely browse off the page to the front page, blog listing
  //        or other pages?

  Ok(())
}

fn main() -> Result<(), GooseError> {
    let _goose_metrics = GooseAttack::initialize()?
        .register_taskset(taskset!("Load blog")
            .register_task(task!(tag1blog).set_name("decoupling apps"))
        )
        .execute()?
        .print();

    Ok(())
}

