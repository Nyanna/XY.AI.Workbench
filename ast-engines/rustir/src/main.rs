fn main() {
    let code = rustir::run(
        std::env::args().skip(1),
        &mut std::io::stdout(),
        &mut std::io::stderr(),
    );
    std::process::exit(code);
}
