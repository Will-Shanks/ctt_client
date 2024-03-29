use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::Shell;
use clap_mangen::Man;
use std::{
    env,
    fs::File,
    io::Error,
    path::{Path, PathBuf},
};

include!("src/cli.rs");

fn build_shell_completion(outdir: &Path) -> Result<(), Error> {
    let mut app = Cli::command();
    app.build();
    let outdir = outdir.join("bash-completions/completions");
    std::fs::create_dir_all(&outdir).unwrap();

    generate_to(Shell::Bash, &mut app, "ctt", outdir)?;

    Ok(())
}

fn build_manpages(outdir: &Path) -> Result<(), Error> {
    let mut app = Cli::command();
    app.build();
    let outdir = outdir.join("man/man1");
    std::fs::create_dir_all(&outdir).unwrap();

    let file = Path::new(&outdir).join("ctt.1");
    let mut file = File::create(file)?;

    Man::new(app.clone()).render(&mut file)?;

    for sub in app.get_subcommands() {
        let file = Path::new(&outdir).join(format!("ctt-{}.1", sub));
        let mut file = File::create(&file)?;

        Man::new(sub.clone()).render(&mut file)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=man");

    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    // Create `target/assets/` folder.
    let out_path = PathBuf::from(outdir);
    let mut path = out_path.ancestors().nth(4).unwrap().to_owned();
    path.push("assets");
    std::fs::create_dir_all(&path).unwrap();

    build_shell_completion(&path)?;
    build_manpages(&path)?;

    Ok(())
}
