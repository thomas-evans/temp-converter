# to run code coverage in powershell:

# set the execution policy:
# https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_execution_policies

# install grcov
    # cargo install grcov
# install llvm-tools
    # rustup component add llvm-tools

# cleanup previous builds
cargo clean
Remove-Item -Path ./*.profraw | Out-Null
# generate annotated test results
$env:CARGO_INCREMENTAL=0; $env:RUSTFLAGS='-Cinstrument-coverage'; $env:LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw'; cargo test
# create windows directory
New-Item -Path "./target/debug/" -Name "coverage" -ItemType "directory" | Out-Null
# generate lcov file from
grcov . -s . --binary-path ./target/debug/ -t lcov --branch --ignore-not-existing -o ./target/debug/coverage/lcov.info