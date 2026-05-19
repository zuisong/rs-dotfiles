# rs-dotfiles

This is a **Rust port** of the original [`dotfiles`](https://github.com/rhysd/dotfiles) command, a minimalist symlink manager designed to handle your dotfiles repository and configurations with ease.

## Goals

- **One binary executable**: Zero-dependency deployment. Just drop the binary into your `$PATH`.
- **Do one thing and do it well**: Focuses strictly on managing dotfiles symlinks.
- **Zero dependencies**: No external runtime or system dependencies like `git` required.
- **Sensible defaults**: Pre-defined mappings for common configuration files on macOS, Linux, and Windows.
- **Rust Powered**: Enhanced type safety, memory security, and performance.

## ⚠️ Differences from original Go version

This Rust port is a simplified version of the original tool and differs in the following ways:

- **No `clone` command**: You should clone your dotfiles repository manually using `git clone`.
- **No `update` command**: You should run `git pull` inside your dotfiles repository manually to keep it updated.
- **No `selfupdate` command**: Use your package manager or `cargo install` to update the tool.
- **Focus on Symlinking**: The tool focuses entirely on managing the links between your repository and your home directory.

## Getting Started

1. **Install via Cargo**:
   ```sh
   cargo install --path .
   ```

2. **Prepare your dotfiles**:
   Manually clone your dotfiles repository and enter the directory.

3. **Check Mappings**:
   ```sh
   rs-dotfiles link --dry
   ```

4. **Apply Links**:
   ```sh
   rs-dotfiles link
   ```

## Usage

```sh
rs-dotfiles <SUBCOMMAND> [ARGS]
```

### Subcommands

- `link`: Create symbolic links based on `mappings.json` and defaults.
  - `--dry`: Preview changes without applying them.
- `list`: Show all current symbolic links managed by this tool.
- `clean`: Remove all symbolic links created by this tool.
- `completion <SHELL>`: Generate shell completion script for bash, zsh, fish, powershell, or elvish.
- `version`: Show the current version.

## Shell Completion

To enable shell completion, add the corresponding line to your shell configuration file:

### Zsh
```sh
source <(rs-dotfiles completion zsh)
```

### Bash
```sh
source <(rs-dotfiles completion bash)
```

### Fish
```sh
rs-dotfiles completion fish | source
```

## Configuration

Custom mappings can be defined in `.dotfiles/mappings.json` within your repository:

```json
{
    ".vimrc": "~/.vimrc",
    ".config/nvim": "~/.config/nvim",
    "bashrc": ["~/.bashrc", "~/.bash_profile"]
}
```

## Development

### Build and Test
```sh
cargo build --release
cargo test
```

## Credits

This project is a port of the original [rhysd/dotfiles](https://github.com/rhysd/dotfiles) written in Go.

## License

MIT License.
