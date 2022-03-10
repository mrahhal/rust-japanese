param (
	[ValidateSet("build", "test", "docs", "format", "lint")]
	[string]$Script = ""
)

if ($Script -eq "") {
	$Script = "build"
}

if ($Script -eq "build") {
	cargo build
}

if ($Script -eq "test") {
	cargo test
}

if ($Script -eq "docs") {
	cargo doc
}

if ($Script -eq "format") {
	cargo fmt
}

if ($Script -eq "lint") {
	cargo clippy
}
