#![deny(warnings)]

use std::{fs, path::Path};

use chrono::{TimeZone, Utc};
use exitfailure::ExitFailure;
use failure::{bail, SyncFailure};
use fs_extra::dir::{self, CopyOptions};
use serde_derive::{Deserialize, Serialize};
use tera::{Context, Tera};

const MAX_DESCRIPTION_SIZE: usize = 280; // characters (assuming ASCII)

#[derive(Deserialize, Serialize)]
struct Project {
    name: String,
    website: String,
    author: String,
    author_website: String,
    description: String,
    image: Option<String>,
    video: Option<Vec<String>>,
}

fn main() -> Result<(), ExitFailure> {
    Ok(run()?)
}

fn run() -> Result<(), failure::Error> {
    let tera = Tera::new("templates/**/*.html").map_err(SyncFailure::new)?;
    let mut projects: Vec<Project> = serde_yaml::from_str(&fs::read_to_string("data.yml")?)?;

    // rotate the project list so that the first entry is not always shown at the top
    // (we rebuild the site on a daily basis)
    let start = Utc.with_ymd_and_hms(2019, 03, 04, 0, 0, 0).unwrap();
    let nprojects = projects.len();
    projects.rotate_left(((Utc::now() - start).num_days() as usize) % nprojects);

    for project in &mut projects {
        if project.description.len() > MAX_DESCRIPTION_SIZE {
            bail!(
                "{}'s project description is longer than {} characters",
                project.name,
                MAX_DESCRIPTION_SIZE
            );
        }

        match (&mut project.image, &mut project.video) {
            (Some(_), Some(_)) => bail!("{} contains both a video and an image", project.name),
            (None, None) => bail!("{} is missing a video or image", project.name),
            (Some(image), None) => {
                if !image.starts_with("http") {
                    // this is a filename / local file; prepend `assets/`
                    *image = format!("assets/{}", image);
                }
            }
            (None, Some(video)) => {
                for source in video {
                    if !source.starts_with("http") {
                        // this is a filename / local file; prepend `assets/`
                        *source = format!("assets/{}", source);
                    }
                }
            }
        }
    }

    let mut context = Context::new();
    context.insert("projects", &projects);
    let index = tera
        .render("index.html", &context)
        .map_err(SyncFailure::new)?;

    fs::remove_dir_all("public").ok();
    fs::create_dir("public")?;
    if Path::new("assets").exists() {
        dir::copy("assets", "public/", &CopyOptions::new())?;
    }
    dir::copy("css", "public/", &CopyOptions::new())?;
    fs::write("public/index.html", index)?;

    Ok(())
}
