all: schema csharp perl service

schema:
    cd tools && cargo run --bin schema_export

csharp:
    cd tools && cargo run --bin generate_csharp_from_rust
    cd csharp-bindings && dotnet build

perl:
    cd tools && cargo run --bin generate_perl_from_rust
    cd perl-bindings && perl Makefile.PL && make

service:
    cd service && cargo build
Run:

bash
make all
You now have:

Rust core crate.

C#/.NET models + HTTP client.

Perl module + HTTP client.

HTTP/HTTPS service exposing glyphs/deities.
