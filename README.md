# 📦 clean-repo

`clean-repo` is a command-line tool that deletes files and directories ignored by your project's `.gitignore`. It helps clean up temporary, build, or generated files before committing or packaging your repository.

---

## 🚀 Features

* Cleans files based on `.gitignore`
* Supports `--dry-run` for previewing deletions
* Easy installation with apt

---

## 🛠 Installation

### Option 1 — Install via APT (recommended)

1. Add the APT repository:

   ```bash
   echo "deb [trusted=yes] https://msaeedsaeedi.github.io/clean-repo stable main" \
     | sudo tee /etc/apt/sources.list.d/clean-repo.list
   ```

2. Update and install:

   ```bash
   sudo apt update
   sudo apt install clean-repo
   ```

### Option 2 — Manual Installation

1. Download the `.deb` package from the Releases page.
2. Install the package:

   ```bash
   sudo dpkg -i clean-repo.deb
   sudo apt --fix-broken install
   ```

---

## 🔧 Usage

* Clean all ignored files:

  ```bash
  clean-repo
  ```

* Dry run (preview deletions):

  ```bash
  clean-repo --dry-run
  ```

---

## 📝 License

MIT License. Feel free to use, modify, and distribute.

---

## 💬 Feedback & Contributions

Issues and PRs are welcome. If you have suggestions or feature requests, feel free to open an issue.
