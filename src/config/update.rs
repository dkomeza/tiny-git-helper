use self_update::backends::github::ReleaseList;

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
                "\n$cg$b `📦 New Update Available`\n\
                 &> $cy `{}` $cw `➜` $cg$b `{}`\n\
                 &> Run $cc$i `tgh update` $cw `to upgrade`\n",
                current_version, release.version
            );
            return Ok(update_msg);
        }
    }

    Ok("".into())
}

pub fn perform_self_update() -> Result<(), Box<dyn std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("dkomeza")
        .repo_name("tiny-git-helper")
        .bin_name("tgh")
        .show_download_progress(true)
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()?
        .update()?;

    println!("Updated to version: {}", status.version());
    Ok(())
}
