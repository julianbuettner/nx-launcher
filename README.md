# NX Launcher

Starting nx targets from the command line sucks. So here is a launcher to be used together with fzf.

## Installation
```bash
cargo build --release
cp ./target/release/nx-launcher /usr/bin/

# Bash
alias fnx="npx nx run $(nx-launcher | fzf)"

# Fish
alias --save fnx "npx nx run (nx-launcher | fzf)"
```


