# NX Launcher

Starting nx targets from the command line sucks. So here is a launcher (fast target lister) to be used together with fzf or fish.


## Installation
```bash
cargo build --release
cp ./target/release/nx-launcher /usr/bin/
```

Test it with
```bash
cd /my/nx/monorepo
nx-launcher
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


### Disclaimer
The first version of the project was vibe coded.
