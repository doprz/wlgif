# Contributing to `wlgif`

Contributions are welcome! This project aims to stay focused on its core purpose while remaining open to improvements whether it's bug reports, feature requests, or code contributions.

## Guidelines

**Before submitting:**
1. **Check existing issues** - Your idea might already be discussed
2. **Open an issue first** - For non-trivial/large changes, discuss the approach before coding
3. **Keep it focused** - New features should enhance the screen-to-GIF workflow, not add unrelated functionality
4. **Maintain composability** - Don't break scripting/piping workflows
5. **Test thoroughly** - Run `cargo test` and test on an actual Wayland session
6. **Format your code** - Run `cargo fmt` before committing
7. **Use Conventional Commits** - This project uses [Conventional Commits](https://www.conventionalcommits.org/)
7. **Write a clear PR description** - Explain *why*, not just *what*

**Good fit for contribution:**
- Recording controls (pause, countdown, visual feedback)
- Performance improvements
- Better error messages
- Shell completions (bash, zsh, fish)
- GUI (feature coming soon)

**Not a good fit:**
- Image editing features (crop, rotate, filters, etc...) - use existing image tools
- Video editing features (trim, concatenate, etc...) - use existing video tools
- Format conversion unrelated to screen capture - use ffmpeg directly

When in doubt, open an issue to discuss!

## Testing

Test on a real Wayland session with your compositor of choice. The core workflow to verify:
1. Region selection works and cancels cleanly
2. Recording captures the correct region
3. GIF output is properly encoded

Alternatively, use the provided [NixOS VMs](HACKING.md#nixos-virtual-machines) for testing in an isolated and reproducible environment.

## Conventional Commits & Branch Naming Convention

We follow the **Conventional Commits** specification for commit messages. This ensures a consistent commit history and enables automated versioning and changelog generation.

### Commit Messages

Follow this structure for commit messages:

```
<type>(<scope>): <subject>
```

Where:

- **type**: One of the following:
    - `feat`: A new feature
    - `fix`: A bug fix
    - `docs`: Documentation updates
    - `style`: Code formatting changes (whitespace, semicolons, etc.)
    - `refactor`: Code restructuring (without adding features or fixing bugs)
    - `test`: Adding or modifying tests
    - `chore`: Maintenance tasks or build process changes

Example:

```
feat(build): add build revision
chore: format workspace
```

### Branch Naming

Branch names should follow the format:

```
<type>/<short-description>
```

Examples:

- `feat/dual-backends`
- `fix/integration-tests`
- `docs/update-readme`
