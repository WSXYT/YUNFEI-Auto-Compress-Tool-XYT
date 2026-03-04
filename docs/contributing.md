# Contributing Guide

Thank you for your interest in contributing to YUNFEI Auto Compress Tool! This guide will help you get started.

## How to Contribute

1. **Fork** the repository
2. **Create a branch** for your feature or fix: `git checkout -b feature/my-feature`
3. **Make your changes** following the coding standards below
4. **Test** your changes locally with `npm run tauri dev`
5. **Commit** with clear, descriptive messages
6. **Push** to your fork and open a **Pull Request**

## Pull Request Process

1. Ensure your PR targets the `main` branch
2. Provide a clear description of the changes and their purpose
3. Include screenshots for UI changes
4. Verify the app builds successfully: `npm run tauri build`
5. Keep PRs focused — one feature or fix per PR
6. Respond to review feedback promptly

## Coding Standards

### General

- Write clear, self-documenting code
- Keep functions small and focused
- Add comments only where logic is non-obvious

### TypeScript / Vue (Frontend)

- Use TypeScript strict mode — avoid `any` types
- Follow Vue 3 Composition API patterns
- Use `useAppState` composable for state management — do not introduce additional stores
- Use Tailwind CSS utility classes for styling
- Component naming: `PascalCase` with descriptive `Section` / `Dialog` suffixes

### Rust (Backend)

- Follow standard Rust conventions (`snake_case` for functions, `PascalCase` for types)
- Handle errors explicitly — avoid `unwrap()` in production paths
- Keep command handlers thin — delegate logic to dedicated modules
- Use `Arc<Mutex<>>` for shared state access

### Commit Messages

- Use present tense: "Add feature" not "Added feature"
- Keep the subject line under 72 characters
- Reference issue numbers where applicable: `Fix #42`

## Issue Reporting

When reporting a bug, please include:

1. **System info** — OS version, app version
2. **Steps to reproduce** — Detailed steps to trigger the issue
3. **Expected behavior** — What you expected to happen
4. **Actual behavior** — What actually happened
5. **Logs** — Any relevant log output from the app or terminal
6. **Screenshots** — If applicable, especially for UI issues

### Feature Requests

- Search existing issues first to avoid duplicates
- Describe the use case and expected behavior clearly
- Explain why this feature would be useful to other users

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](../LICENSE).
