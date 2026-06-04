# Create GitHub repo and push

Target:

```text
Repo:   ProfRandom92/comptext-cli
Crate:  comptext-cli
Binary: ctxt
```

Create and push with GitHub CLI:

```bash
git init
git branch -M main
git add .
git commit -m "Initialize CompText CLI scaffold"
gh repo create ProfRandom92/comptext-cli --public --source=. --remote=origin --push
```

After push, open the repo in Antigravity and run:

```text
prompts/AGY_00_REPO_GENESIS.md
```

Then continue with:

```text
prompts/AGY_01_CONTEXT_PACK.md
```
