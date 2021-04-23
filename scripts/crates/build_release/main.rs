use clap::Clap;
use duct::cmd;
use indoc::formatdoc;
use log::info;
use std::{
	io::prelude::*,
	path::{Path, PathBuf},
};
use tangram_error::Result;
use tangram_script_build::{target_file_names, Target};

#[derive(Clap)]
pub struct Args {
	#[clap(long, env)]
	version: String,
}

pub fn main() -> Result<()> {
	let env = env_logger::Env::new().filter_or("LOG", "info");
	env_logger::Builder::from_env(env)
		.format(|buf, record| writeln!(buf, "{}", record.args()))
		.init();

	let args = Args::parse();

	let tangram_path = std::env::current_dir()?;
	let dist_path = tangram_path.join("dist");

	info!("clean and create release directory");
	let release_path = dist_path.join("release");
	clean_and_create(&release_path)?;

	info!("tangram_cli");
	for target in &[
		Target::X8664UnknownLinuxGnu,
		Target::X8664UnknownLinuxMusl,
		Target::X8664AppleDarwin,
		Target::AArch64AppleDarwin,
		Target::X8664PcWindowsMsvc,
		Target::X8664PcWindowsGnu,
	] {
		let tangram_cli_file_name = target_file_names(*target).tangram_cli_file_name;
		let tangram_cli_path = dist_path.join(target.as_str()).join(tangram_cli_file_name);
		let output_path =
			release_path.join(format!("tangram_cli_{}_{}.tar.gz", args.version, target));
		let inputs = vec![(
			tangram_cli_path.clone(),
			PathBuf::from(tangram_cli_file_name),
		)];
		tar(inputs, &output_path)?;
	}

	info!("deb");
	#[allow(clippy::single_element_loop)]
	for target in &[Target::X8664UnknownLinuxMusl] {
		// Create the deb directory.
		let deb_path = dist_path.join("deb");
		clean_and_create(&deb_path)?;
		// Create /usr/bin in the deb directory.
		let bin_path = deb_path.join("usr/bin");
		std::fs::create_dir_all(&bin_path)?;
		// Copy the tangram cli to the deb/usr/bin.
		let tangram_cli_file_name = target_file_names(*target).tangram_cli_file_name;
		let tangram_cli_path = dist_path.join(target.as_str()).join(tangram_cli_file_name);
		std::fs::copy(tangram_cli_path, bin_path.join(tangram_cli_file_name))?;
		// Create the control file.
		let debian_path = deb_path.join("DEBIAN");
		std::fs::create_dir_all(&debian_path)?;
		let control_path = debian_path.join("control");
		let architecture = match target {
			Target::X8664UnknownLinuxMusl => "amd64",
			_ => unreachable!(),
		};
		let control = formatdoc! {
			r#"
				Package: tangram
				Architecture: {}
				Version: {}
				Maintainer: Tangram <root@tangram.xyz>
				Homepage: https://www.tangram.xyz
				Description: Tangram is an automated machine learning framework designed for programmers.
			"#,
			architecture,
			args.version,
		};
		std::fs::write(&control_path, &control)?;
		// Run dpkg-deb
		let deb_file_name = format!("tangram_{}_{}.deb", args.version, architecture);
		let deb_output_path = release_path.join(&deb_file_name);
		cmd!("dpkg-deb", "--build", &deb_path, &deb_output_path).run()?;
		std::fs::remove_dir_all(&deb_path)?;
	}

	info!("rpm");
	#[allow(clippy::single_element_loop)]
	for target in &[Target::X8664UnknownLinuxMusl] {
		// Create the rpm directory.
		let rpm_path = dist_path.join("rpm");
		clean_and_create(&rpm_path)?;
		for subdir in &["BUILD", "BUILDROOT", "RPMS", "SOURCES", "SPECS", "SRPMS"] {
			std::fs::create_dir(rpm_path.join(subdir))?;
		}
		// Make the tar.
		let tangram_cli_file_name = target_file_names(*target).tangram_cli_file_name;
		let tangram_cli_path = dist_path.join(target.as_str()).join(tangram_cli_file_name);
		let tangram_path_in_tar = PathBuf::from(format!("tangram-{}/tangram", args.version));
		let sources_path = rpm_path.join("SOURCES");
		let tar_path = sources_path.join("tangram.tar.gz");
		tar(vec![(tangram_cli_path, tangram_path_in_tar)], &tar_path)?;
		// Write the spec file.
		let spec = formatdoc! {
			r#"
				Name: tangram
				Version: {}
				Release: 1
				Summary: Tangram is an automated machine learning framework designed for programmers.
				License: MIT
				Source0: tangram.tar.gz

				%description
				%summary

				%prep
				%setup -q

				%install
				mkdir -p %buildroot/%_bindir
				install -m 755 tangram %buildroot/%_bindir/tangram

				%files
				%attr(0755, root, root) %_bindir/tangram
			"#,
			args.version,
		};
		let spec_path = rpm_path.join("SPECS/tangram.spec");
		std::fs::write(&spec_path, spec)?;
		// Run rpmbuild.
		let target = match target {
			Target::X8664UnknownLinuxMusl => "x86_64",
			_ => unreachable!(),
		};
		cmd!(
			"rpmbuild",
			"-D",
			format!("_topdir {}", rpm_path.display()),
			"--target",
			target,
			"-bb",
			spec_path,
		)
		.run()?;
		// Move the rpm to the release directory.
		let src_rpm_file_name = format!("tangram-{}-1.{}.rpm", args.version, target);
		let dst_rpm_file_name = format!("tangram_{}_{}.rpm", args.version, target);
		std::fs::copy(
			rpm_path.join("RPMS").join(target).join(&src_rpm_file_name),
			release_path.join(&dst_rpm_file_name),
		)?;
		std::fs::remove_dir_all(rpm_path)?;
	}

	info!("container");
	let dockerfile_path = tangram_path.join("Dockerfile");
	let target = Target::X8664UnknownLinuxMusl;
	let tangram_cli_file_name = target_file_names(target).tangram_cli_file_name;
	let tangram_cli_path = dist_path
		.strip_prefix(&tangram_path)
		.unwrap()
		.join(target.as_str())
		.join(tangram_cli_file_name);
	let dockerfile = formatdoc! {
		r#"
			FROM docker.io/alpine
			WORKDIR /
			COPY {} .
			ENTRYPOINT ["/tangram"]
		"#,
		tangram_cli_path.display(),
	};
	std::fs::write(&dockerfile_path, &dockerfile)?;
	let tag = format!("docker.io/tangramxyz/tangram:{}", args.version);
	cmd!("podman", "build", "-t", tag, &tangram_path).run()?;
	std::fs::remove_file(&dockerfile_path)?;

	info!("libtangram");
	for target in &[
		Target::X8664UnknownLinuxGnu,
		Target::X8664UnknownLinuxMusl,
		Target::X8664AppleDarwin,
		Target::AArch64AppleDarwin,
		Target::X8664PcWindowsMsvc,
		Target::X8664PcWindowsGnu,
	] {
		let target_file_names = target_file_names(*target);
		let target_path = dist_path.join(target.as_str());
		let output_path =
			release_path.join(format!("libtangram_{}_{}.tar.gz", args.version, target));
		let inputs = vec![
			(
				target_path.join(target_file_names.tangram_h_file_name),
				PathBuf::from(target_file_names.tangram_h_file_name),
			),
			(
				target_path.join(target_file_names.libtangram_dynamic_file_name),
				PathBuf::from(target_file_names.libtangram_dynamic_file_name),
			),
			(
				target_path.join(target_file_names.libtangram_static_file_name),
				PathBuf::from(target_file_names.libtangram_static_file_name),
			),
		];
		tar(inputs, &output_path)?;
	}

	Ok(())
}

fn clean_and_create(path: &Path) -> Result<()> {
	let path_exists = std::fs::metadata(path).map(|m| m.is_dir()).unwrap_or(false);
	if path_exists {
		std::fs::remove_dir_all(path)?;
	}
	std::fs::create_dir_all(path)?;
	Ok(())
}

fn tar(input_paths: Vec<(PathBuf, PathBuf)>, output_path: &Path) -> Result<()> {
	let output_file = std::fs::File::create(output_path)?;
	let gz = flate2::write::GzEncoder::new(output_file, flate2::Compression::default());
	let mut tar = tar::Builder::new(gz);
	for (path, name) in input_paths.iter() {
		tar.append_path_with_name(path, name)?;
	}
	tar.finish()?;
	Ok(())
}
