use self_update::backends::github::{ReleaseList, Update};

pub async fn check_for_updates() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let update_check = tokio::task::spawn_blocking(|| tokio_check_for_updates())
        .await
        .expect("Blocking task panicked");

    update_check
}

fn tokio_check_for_updates() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let current_version = env!("CARGO_PKG_VERSION");

    let releases = ReleaseList::configure()
        .repo_owner("dkomeza")
        .repo_name("tiny-git-helper")
        .build()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
        .fetch()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

    let latest_release = releases
        .iter()
        .filter(|r| !r.version.contains("alpha") && !r.version.contains("beta"))
        .max_by(|a, b| {
            let version_a =
                semver::Version::parse(&a.version).unwrap_or(semver::Version::new(0, 0, 0));
            let version_b =
                semver::Version::parse(&b.version).unwrap_or(semver::Version::new(0, 0, 0));
            version_a.cmp(&version_b)
        });

    if let Some(release) = latest_release {
        let current = semver::Version::parse(current_version)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        let latest = semver::Version::parse(&release.version)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        if latest > current {
            let update_msg = format!(
                "\n$cg$b `📦 New Update Available`
                 &> $cy `{}` $cw `➜` $cg$b `{}`
                 &> Run $cc$i `tgh update` $cw `to upgrade`\n",
                current_version, release.version
            );
            return Ok(update_msg);
        }
    }

    Ok("".into())
}

pub async fn perform_self_update() {
    use crate::view::printer;
    let current_ver = env!("CARGO_PKG_VERSION");

    printer(format!(
        "\n$cb$b `🔍 Checking for updates...`\n&> $cw `Current version:` $cy `{}`\n",
        current_ver
    ));

    let update_available = check_for_updates().await;

    match update_available {
        Ok(msg) => {
            if msg.is_empty() {
                printer(format!(
                    "$cg$b `✔ You are already up to date.`\n&> $cw `Version:` $cg `{}`\n",
                    current_ver
                ));
                return;
            } else {
                printer("\n$cy$b `⬇ Update found! Starting download...`\n");
            }
        }
        Err(e) => {
            printer(format!(
                "\n$cr$b `✖ Failed to check for updates`\n&> $cr `Error:` $cw `{}`\n",
                e
            ));
            return;
        }
    }

    let update_result = tokio::task::spawn_blocking(|| execute_update_logic())
        .await
        .expect("Blocking task panicked");

    // 4. Report Final Status
    match update_result {
        Ok(new_version) => {
            printer(format!(
                "\n$cg$b `✨ Update Successful!`\n&> $cw `New version:` $cg `{}`\n&> $cw `Please restart the terminal to use the new version.`\n",
                new_version
            ));
        }
        Err(err) => {
            printer(format!(
                "\n$cr$b `✖ Update Failed`\n&> $cr `Reason:` $cw `{}`\n",
                err
            ));
        }
    }
}

// The internal blocking logic
fn execute_update_logic() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let status = Update::configure()
        .repo_owner("dkomeza")
        .repo_name("tiny-git-helper")
        .bin_name("tgh")
        .show_download_progress(true)
        .current_version(env!("CARGO_PKG_VERSION"))
        .no_confirm(true)
        .build()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
        .update()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

    // Return the new version string
    Ok(status.version().to_string())
}
