CONTRIBUTORS_GUIDELINE.md

# 🌟 Contributing Guide | TrustBridge

We’re thrilled that you’re interested in contributing to TrustBridge! This guide will help you understand the process and ensure smooth collaboration for all contributors.

---

## 🛠️ Steps to Contribute

### 1️⃣ Fork the Repository
- Create a **GitHub account** if you don’t already have one.
- Navigate to the **TrustBridge repository’s GitHub page**.
- Click the **Fork** button in the top-right corner to create a copy of the repository under your account.

---

### 2️⃣ Clone the Repository
- Clone the forked repository to your local machine using the following command:
  ```bash
  git clone https://github.com/YOUR_USERNAME/trustbridge.git


  Replace `YOUR_USERNAME` with your GitHub username.

---

### 3️⃣ Create a New Branch
- Follow **branch naming conventions**:
  - Use `feat/<feature-name>` for new features.
  - Use `fix/<bug-name>` for bug fixes.
  - Use `docs/<doc-update>` for documentation updates.
- Create a new branch for your changes:
  ```bash
  git checkout -b feat/<feature-name>
  ```
  Example: `feat/update-readme` or `fix/authentication-bug`.

---

### 4️⃣ Make Changes and Commit
- Implement your changes locally.
- Use **atomic commits**—each commit should represent a single, logical change.
- Write clear and descriptive commit messages using the following format:
  ```bash
  git add .
  git commit -m "type(scope): description"
  ```
  Examples:
  - `feat(authentication): add support for OAuth login`
  - `fix(api): resolve 404 error in user endpoint`
  - `docs(readme): update contribution guidelines`

---

### 5️⃣ Test Locally
- Before pushing your changes, ensure the project runs correctly:
  - Follow the project-specific setup instructions.
  - Run existing tests and add new ones, if necessary.

---

### 6️⃣ Push Your Changes
- Push your changes to your forked repository:
  ```bash
  git push origin your-branch-name
  ```
- **Pre-push hooks**: TrustBridge uses Husky to enforce coding standards (e.g., linting and formatting). If you encounter errors, fix them before retrying the push.

---

### 7️⃣ Create a Pull Request (PR)
- Navigate to your forked repository on GitHub.
- Click **New Pull Request** and select your branch to merge into the `main` branch of the original repository.
- Fill out the provided **Pull Request Template** with all required details:
  - Include a clear description of your changes.
  - Reference any related issues.

---

## 🧰 Additional Notes

- **Follow Git Guidelines**:
  - Ensure proper branch naming, commit messages, and overall contribution standards.
- **Respect code style and standards**:
  - Use the repository’s coding conventions.
  - Run `npm run lint` and `npm run format` to ensure consistency.
- **Collaborate effectively**:
  - Be responsive to feedback from maintainers.
  - Respect requested changes to your PR.
- **Testing requirements**:
  - Include automated tests for any new features or bug fixes.
  - Ensure all tests pass before submitting your PR.

---


Thank you for contributing to TrustBridge! Together, we can build something amazing. 🚀✨

