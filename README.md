# NX Launcher

Starting nx targets from the command line sucks. So here is a launcher to be used together with fzf.

## Installation
```bash
cargo build --release
cp ./target/release/nx-launcher /usr/bin/
```

### Fish

```fish
alias --save fnx "npx nx run (nx-launcher | fzf)"
```

```fish
# ~/.fish/functions/nx.fish
complete \
    -c nx \
    -n '__fish_seen_subcommand_from run' \
    -a '(nx-launcher)' \
    -d 'nx run target'
```

# Bash
```bash
# Append to ~/.bashrc
alias fnx="npx nx run $(nx-launcher | fzf)"
```

