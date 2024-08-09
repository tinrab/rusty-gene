#!/bin/bash

set -euo pipefail
current_path="$(realpath $0)"
current_dir="$(dirname $current_path)"

function lint() {
	cargo fmt --all -- --check

	disallow=(
		warnings
		unsafe_code
		trivial_casts
		trivial_numeric_casts
		# unreachable_pub
		# missing_docs
		unused_extern_crates
		unused_import_braces
		unused_qualifications
		clippy::all
		clippy::correctness
		clippy::suspicious
		clippy::complexity
		clippy::perf
		clippy::style
		clippy::pedantic
		# clippy::nursery
		# clippy::missing_errors_doc
		# clippy::missing_panics_doc
	)
	allow=(
		unused_braces
		clippy::module_name_repetitions
		clippy::cast_possible_truncation
		clippy::cast_possible_wrap
		clippy::must_use_candidate
		clippy::cast_sign_loss
		clippy::too_many_lines
		clippy::needless_pass_by_value
		clippy::struct_excessive_bools
		clippy::missing_errors_doc
		clippy::missing_panics_doc
		clippy::struct_field_names
	)

	cargo clippy --workspace --all-targets --all-features \
		-- ${disallow[@]/#/-D } ${allow[@]/#/-A }
}

function help() {
	echo "Usage: $(basename "$0") [OPTIONS]

Commands:
  lint           Run lints
  help           Show help
"
}

if [[ $1 =~ ^(lint|help)$ ]]; then
	"$@"
else
	help
	exit 1
fi
