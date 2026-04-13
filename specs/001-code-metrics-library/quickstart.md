# Quickstart: Arborist

**Branch**: `001-code-metrics-library` | **Date**: 2026-03-27

## Prerequisites

- Rust toolchain (edition 2024)
- C compiler (required by arborium language bindings for native compilation)

## Add to your project

Arborist's public feature flags map to `arborium/lang-*` features internally.
The `typescript` feature enables both `lang-typescript` and `lang-tsx` so `.ts`
and `.tsx` files continue to share the same `Language::TypeScript` API.

```toml
# Cargo.toml — default features (Rust, Python, JS, TS, Java, Go)
[dependencies]
arborist-metrics = "0.1"

# Or select specific languages
[dependencies]
arborist-metrics = { version = "0.1", default-features = false, features = ["rust", "python"] }

# Or all 12 languages (Tier 1 + Tier 2)
[dependencies]
arborist-metrics = { version = "0.1", features = ["all"] }
```

## Analyze a file

```rust
use arborist::{analyze_file, FileReport};

fn main() -> Result<(), arborist::ArboristError> {
    let report: FileReport = analyze_file("src/main.rs")?;

    println!("File: {} ({:?})", report.path, report.language);
    println!("Total cognitive: {}, SLOC: {}", report.file_cognitive, report.file_sloc);

    for func in &report.functions {
        println!("  {} (lines {}-{}): cognitive={}, cyclomatic={}, sloc={}",
            func.name, func.start_line, func.end_line,
            func.cognitive, func.cyclomatic, func.sloc);
    }

    Ok(())
}
```

## Analyze source code from memory

```rust
use arborist::{analyze_source, Language};

let source = r#"
def hello(name):
    if name:
        print(f"Hello, {name}!")
    else:
        print("Hello, world!")
"#;

let report = analyze_source(source, Language::Python)?;
// report.functions[0].cognitive == 2 (if + else)
```

## Configure thresholds

```rust
use arborist::{analyze_file_with_config, AnalysisConfig};

let config = AnalysisConfig {
    cognitive_threshold: Some(8),
    ..Default::default()
};

let report = analyze_file_with_config("src/complex.rs", &config)?;

for func in &report.functions {
    if func.exceeds_threshold == Some(true) {
        eprintln!("WARNING: {} has cognitive complexity {} (threshold: 8)",
            func.name, func.cognitive);
    }
}
```

## Serialize to JSON

```rust
let report = analyze_file("src/main.rs")?;
let json = serde_json::to_string_pretty(&report)?;
println!("{}", json);
```

## Development workflow (TDD)

Per the project constitution, development follows test-first, fixture-driven workflow:

1. Create fixture file with known complexity: `tests/fixtures/{language}/example.{ext}`
2. Write test asserting expected metric values
3. Implement or update the `LanguageProfile`
4. Run tests: `cargo test --all-features`
5. Lint: `cargo clippy -- -D warnings`

## Feature flags reference

| Flag | Languages | Internal arborium feature(s) |
|------|-----------|-------------------------------|
| `default` | Rust, Python, JavaScript, TypeScript, Java, Go | Composite |
| `all` | All 12 languages (Tier 1 + Tier 2) | Composite |
| `rust` | Rust only | `lang-rust` |
| `python` | Python only | `lang-python` |
| `javascript` | JavaScript + JSX | `lang-javascript` |
| `typescript` | TypeScript + TSX | `lang-typescript`, `lang-tsx` |
| `java` | Java | `lang-java` |
| `csharp` | C# | `lang-c-sharp` |
| `cpp` | C++ | `lang-cpp` |
| `c` | C | `lang-c` |
| `go` | Go | `lang-go` |
| `php` | PHP | `lang-php` |
| `kotlin` | Kotlin + Kotlin Script | `lang-kotlin` |
| `swift` | Swift | `lang-swift` |
