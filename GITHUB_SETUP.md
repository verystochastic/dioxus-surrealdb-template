# GitHub Setup Guide

Instructions for publishing this template to GitHub.

## Pre-Publish Checklist

Before pushing to GitHub:

- [ ] **Verify all sensitive data removed**
  - [ ] No API keys or secrets in code
  - [ ] No personal information
  - [ ] Database directory (`ideas.db/`) is gitignored
  - [ ] `.env` files are gitignored

- [ ] **Update repository information**
  - [ ] Set your GitHub username in `Cargo.toml`
  - [ ] Set your GitHub username in `README.md`
  - [ ] Update author name in `Cargo.toml`
  - [ ] Update author name in `LICENSE`

- [ ] **Test the template works**
  - [ ] Run `cargo clean`
  - [ ] Run `cargo build` (should complete without errors)
  - [ ] Run `dx serve --platform web` (should start successfully)
  - [ ] Test in browser at http://localhost:8080
  - [ ] Submit a test idea and verify it works

## Initial Git Setup

If you haven't already initialized git:

```bash
# Initialize repository
git init

# Add all files
git add .

# Create initial commit
git commit -m "Initial commit: Dioxus + SurrealDB fullstack template"
```

## Create GitHub Repository

### Option 1: GitHub CLI (Recommended)

```bash
# Install GitHub CLI if needed
# macOS: brew install gh
# Linux: See https://github.com/cli/cli#installation

# Login to GitHub
gh auth login

# Create repository
gh repo create dioxus-surrealdb-template --public --source=. --description="A fullstack Rust web application template using Dioxus and SurrealDB"

# Push code
git push -u origin main
```

### Option 2: GitHub Web Interface

1. Go to https://github.com/new
2. Repository name: `dioxus-surrealdb-template`
3. Description: "A fullstack Rust web application template using Dioxus and SurrealDB"
4. Choose **Public** (so others can use it as a template)
5. **DO NOT** initialize with README, .gitignore, or license (you already have these)
6. Click "Create repository"

Then push your code:

```bash
git remote add origin https://github.com/YOUR_USERNAME/dioxus-surrealdb-template.git
git branch -M main
git push -u origin main
```

## Enable as GitHub Template

After pushing to GitHub:

1. Go to your repository on GitHub
2. Click **Settings** (top right)
3. Check **"Template repository"** under "General"
4. Save changes

Now others can click "Use this template" to create new repositories from it!

## Add Repository Topics

Help others discover your template:

1. Go to your repository on GitHub
2. Click the gear icon next to "About"
3. Add topics:
   - `rust`
   - `dioxus`
   - `surrealdb`
   - `fullstack`
   - `template`
   - `wasm`
   - `webassembly`
   - `mvp`
   - `starter-template`

## Optional: Add Repository Features

### Add a Description
In repository settings → About:
- Description: "Fullstack Rust web app template with Dioxus + SurrealDB. Perfect for MVPs and rapid prototyping."
- Website: (leave blank or add your personal site)
- Topics: (as listed above)

### Enable Discussions
Settings → Features → Check "Discussions"
- Create a "Show and tell" category for projects built with the template

### Add Branch Protection (Optional)
Settings → Branches → Add rule for `main`:
- Require pull request reviews before merging
- Require status checks to pass

## Create a Release

After your first commit:

```bash
# Create and push a tag
git tag -a v0.1.0 -m "Initial template release"
git push origin v0.1.0
```

Then on GitHub:
1. Go to Releases
2. Click "Draft a new release"
3. Choose tag `v0.1.0`
4. Release title: `v0.1.0 - Initial Release`
5. Description:
   ```markdown
   ## Initial Release

   A fullstack Rust web application template featuring:
   - ✅ Dioxus fullstack framework
   - ✅ SurrealDB integration
   - ✅ Server functions
   - ✅ Example CRUD implementation
   - ✅ Production deployment guides

   Perfect for building MVPs and prototypes quickly!
   ```
6. Click "Publish release"

## Promote Your Template

Share on:
- [/r/rust](https://reddit.com/r/rust)
- [Dioxus Discord](https://discord.gg/XgGxMSkvUM)
- [Rust Users Forum](https://users.rust-lang.org/)
- Twitter/X with hashtags: #rustlang #dioxus #surrealdb
- Dev.to blog post explaining the architecture

## Maintenance

### Keep Dependencies Updated

Set up Dependabot:
1. Create `.github/dependabot.yml`:
   ```yaml
   version: 2
   updates:
     - package-ecosystem: "cargo"
       directory: "/"
       schedule:
         interval: "weekly"
   ```

### Monitor Issues

Watch for:
- Users asking questions (answer or improve docs)
- Bug reports (fix and release patch)
- Feature requests (consider adding if valuable)

### Regular Updates

- Update Dioxus version quarterly
- Update README with new features
- Add more examples based on user feedback

## Success Metrics

Track template adoption:
- GitHub stars
- Number of "Used by" repositories
- Issues/discussions activity
- Social media mentions

## Next Steps

After publishing:

1. ✅ Star your own repository (yes, really!)
2. ✅ Share with at least 3 communities
3. ✅ Write a blog post about why you created it
4. ✅ Respond to first issue within 24 hours
5. ✅ Create a "Projects built with this" section in README

Congratulations on publishing your template! 🎉
