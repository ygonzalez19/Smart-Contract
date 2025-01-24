# 🚀 Git Guidelines | TrustBridge

This document outlines our version control practices and guidelines for the [TrustBridge smartcontract](https://github.com/TrustBridgeCR/Smart-Contract). Following these conventions ensures consistent collaboration and maintainable code history.

---

## 🌳 Branching Strategy


### Feature Development
1. Always branch off from `master`
2. Merge back into `master` via pull request
3. Delete feature branch after successful merge (optional)

### Branch Naming Convention
**All branch names must be lowercase and use hyphens for spaces.**

Branch prefixes based on change type:
- `feat/` 🌟: New features (e.g., `feat/user-authentication`)
- `fix/` 🐛: Bug fixes (e.g., `fix/login-validation`)
- `docs/` 📖: Documentation updates (e.g., `docs/api-guide`)
- `style/` 🎨: Code style changes (e.g., `style/code-formatting`)
- `refactor/` ♻️: Code refactoring (e.g., `refactor/payment-system`)
- `perf/` ⚡: Performance improvements (e.g., `perf/query-optimization`)
- `test/` 🧪: Test-related changes (e.g., `test/integration-tests`)
- `build/` 🏗️: Build system changes (e.g., `build/webpack-config`)
- `ci/` 🔄: CI/CD updates (e.g., `ci/deploy-script`)
- `chore/` 🛠️: Maintenance tasks (e.g., `chore/dependency-update`)

---

## ✍️ Commit Message Guidelines

### Format
```
<type>: <short description>

[optional body]
[optional footer]
```

### Commit Types
- `feat` 🌟: New feature addition
- `fix` 🐛: Bug fix
- `docs` 📖: Documentation changes
- `style` 🎨: Code formatting changes
- `refactor` ♻️: Code refactoring
- `perf` ⚡: Performance improvements
- `test` 🧪: Test-related changes
- `build` 🏗️: Build system changes
- `ci` 🔄: CI/CD changes
- `chore` 🛠️: Maintenance work

### Best Practices
1. Keep the subject line under 72 characters
2. Use imperative mood ("add", "update" not "added" or "adds")
3. Add detailed explanation in commit body if needed
4. Reference issues number in commit

### Examples
```
feat: implement two-factor authentication

- Add SMS verification capability
- Create backup code generation
- Implement rate limiting

Closes #123
```

```
Closes #321

fix: resolve race condition in payment processing

Prevents duplicate transactions by adding distributed locking
```

---

## 🔄 Pull Request Workflow

### Creating Pull Requests
1. Create PR from your feature branch to `master`
2. Fill out PR template completely
3. Add relevant labels
4. Request reviews from team members
5. Link related issue number

### PR Requirements
- `cargo build` and `cargo run` both runs correctly
- Code coverage requirements met
- No merge conflicts
- Documentation updated if needed


---

## 🤝 Merging Policies

### Merge Requirements
- All PR requirements met
- Required approvals received
- No unresolved discussions
- Up-to-date with master branch


---

## ⚔️ Conflict Resolution

### Preventing Conflicts
1. Regularly sync with target branch
2. Communicate with team about overlapping work
3. Keep PRs focused and small

### Resolving Conflicts
1. Pull latest changes from master branch
2. Resolve conflicts locally
3. Test thoroughly after resolution
4. Push resolved changes
5. Request fresh review if needed

---

## 📚 Additional Resources

- [Git Documentation](https://git-scm.com/doc)
- [TrustBridge Wiki](https://github.com/TrustBridgeCR)
- [Team Contact List](support@trustbridge.com)


---

## 🤝 Contributing

Thank you for following these guidelines! They help us maintain a clean and efficient development process. If you have suggestions for improvements, please open an [issue](https://github.com/TrustBridgeCR/Smart-Contract/issues) .

Remember: We're building something great together! 🚀
