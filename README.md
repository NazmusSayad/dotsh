# dotsh

Convert dotenv-style key-value pairs into shell-specific environment export commands.

## What it does

Takes input like:

```
CARGO_HOME=C:\Users\Sayad\.cargo
GOROOT=C:\Users\Sayad\AppData\Local\mise\installs\go\1.26.2
JAVA_HOME=C:\Users\Sayad\AppData\Local\mise\installs\java\26.0.1
```

And outputs shell commands:

```powershell
# pwsh
$env:CARGO_HOME = 'C:\Users\Sayad\.cargo'
$env:GOROOT = 'C:\Users\Sayad\AppData\Local\mise\installs\go\1.26.2'
$env:JAVA_HOME = 'C:\Users\Sayad\AppData\Local\mise\installs\java\26.0.1'
```

## Installation

```bash
cargo install --path .
```

Or clone and build:

```bash
git clone <repo>
cd dotsh
cargo build --release
```

## Usage

```
dotsh <shell> <dotenv-line> [<dotenv-line> ...]
```

### Supported shells

| Shell | Output format |
|-------|---------------|
| `bash`, `zsh`, `sh` | `export KEY='VALUE'` |
| `pwsh`, `powershell` | `$env:KEY = 'VALUE'` |
| `fish` | `set -gx KEY "VALUE"` |
| `cmd` | `set "KEY=VALUE"` |

Shell names are case-insensitive.

### Initialize from a directory

```bash
eval "$(dotsh init bash --home "${DOTFILES_DIR:-$HOME/.dotfiles}")"
```

`--home` is required and selects the directory containing your environment and PATH files. It does not change `HOME`. There is no config file, automatic directory discovery, or dependency on a dotfiles layout outside the files below.

The platform suffix is `win` on Windows, `mac` on macOS, `wsl` in WSL, and `linux` on other Linux systems.

| File | Behavior |
|------|----------|
| `.path.<platform>` | Append one directory per line to PATH, skipping duplicates |
| `.env` | Load shared environment values |
| `.env.<platform>` | Override shared environment values for this platform |
| `.local/.env` | Load local overrides last |

Missing files are skipped. Relative PATH entries are kept as written, matching the existing shell behavior. Environment files use the same literal `KEY=VALUE` parsing as the conversion command; variable references and quotes are not expanded or interpreted.

Files are read directly by `dotsh`. For POSIX shells on Windows, nonempty PATH files are converted in one batched `cygpath` invocation. PATH deduplication for POSIX shells uses the shell's live PATH rather than the native Windows process PATH.

All output is generated before anything is printed. Invalid arguments, unreadable files, invalid environment variable names, and failed path conversions exit with an error and no stdout.

The other supported shells can also use `dotsh init <shell> --home <directory>` and evaluate its output with their usual mechanism.

### Basic examples

```bash
# bash
dotsh bash "CARGO_HOME=/home/user/.cargo"
# → export CARGO_HOME='/home/user/.cargo'

# PowerShell
dotsh pwsh "CARGO_HOME=C:\Users\name\.cargo"
# → $env:CARGO_HOME = 'C:\Users\name\.cargo'

# fish
dotsh fish "CARGO_HOME=/home/user/.cargo"
# → set -gx CARGO_HOME "/home/user/.cargo"
```

### Multiple variables

Pass each `KEY=VALUE` as a separate argument:

```bash
dotsh bash "A=1" "B=2" "C=3"
```

Or pipe from another tool:

```powershell
# PowerShell — evaluate mise env directly
Invoke-Expression (& dotsh pwsh @(mise env -D) | Out-String)
```

```bash
# bash — evaluate mise env directly
eval "$(mise env -D | xargs -d '\n' dotsh bash)"
```

```fish
# fish — evaluate mise env directly
mise env -D | dotsh fish | source
```

## PowerShell hook (recommended)

Add to your `$PROFILE`:

```powershell
function dotenv {
    Invoke-Expression (& dotsh pwsh @(mise env -D) | Out-String)
}
```

Then run `dotenv` whenever you want to load your mise environment.

## How it works

1. Reads `KEY=VALUE` arguments
2. Skips empty lines and `#` comments
3. Splits on the first `=` (values containing `=` are preserved)
4. Escapes values for the target shell
5. Prints all commands as a single string

## Testing

```bash
cargo test
```

86 tests (68 unit + 18 integration).

## License

MIT
