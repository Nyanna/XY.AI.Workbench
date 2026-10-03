mod generated;
mod engine;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "convert" {
        if args.len() < 3 {
            eprintln!("usage: convert <directory>");
            std::process::exit(1);
        }
        convert(std::path::Path::new(&args[2]));
        return;
    }
    let port: u16 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or_else(|| std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8787));
    let server = engine::rust_ast_server::RustAstServer::new();
    println!("rust-syn ast engine listening on port {}", port);
    use crate::generated::engine::AppendInfoNodesValidateServer::AppendInfoNodesValidateServer;
    if let Err(e) = server.start(port) {
        eprintln!("server error: {}", e);
        std::process::exit(1);
    }
}

/// Recursively re-parses and re-prints every `.rs` file under `root`, normalising its formatting.
fn convert(root: &std::path::Path) {
    for entry in walk(root) {
        if entry.extension().map(|e| e == "rs").unwrap_or(false) {
            match std::fs::read_to_string(&entry) {
                Ok(source) => match engine::rust_ast_engine::parse_file(&source) {
                    Ok(file) => {
                        let printed = engine::rust_ast_engine::print_file(&file);
                        if let Err(e) = std::fs::write(&entry, printed) {
                            eprintln!("failed to write {}: {}", entry.display(), e);
                        } else {
                            println!("converted {}", entry.display());
                        }
                    }
                    Err(e) => eprintln!("failed to convert {}: {}", entry.display(), e),
                },
                Err(e) => eprintln!("failed to read {}: {}", entry.display(), e),
            }
        }
    }
}

fn walk(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    if let Ok(read) = std::fs::read_dir(dir) {
        for entry in read.flatten() {
            let path = entry.path();
            if path.is_dir() {
                out.extend(walk(&path));
            } else {
                out.push(path);
            }
        }
    }
    out
}
