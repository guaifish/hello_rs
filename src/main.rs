use backtrace;

fn main() {
    backtrace::trace(|frame| {
        let ip = frame.ip();
        println!("ip: {:?}", ip);
        let symbol_address = frame.symbol_address();
        println!("symbol_address: {:?}", symbol_address);

        // Resolve this instruction pointer to a symbol name
        backtrace::resolve_frame(frame, |symbol| {
            if let Some(name) = symbol.name() {
                println!("name: {:?}", name);
            }
            if let Some(filename) = symbol.filename() {
                println!("filename {:?}", filename);
            }
        });
        println!("\n");
        true // keep going to the next frame
    });
}