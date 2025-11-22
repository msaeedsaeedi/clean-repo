# 📦 clean-repo

`clean-repo` is a command-line tool that deletes files and directories ignored by your project's `.gitignore`. It helps clean up temporary, build, or generated files before committing or packaging your repository.

---

## 🚀 Features

* Cleans files based on `.gitignore`
* Safe dry-run mode by default (shows what would be deleted)
* Pattern exclusion to keep specific files
* Verbose and quiet modes
* Comprehensive man page documentation
* Easy installation with apt

---

## 🛠 Installation

### Option 1 — Quick Install (recommended)

Run the installation script:

```bash
curl -fsSL https://repo.msaeedsaeedi.com/clean-repo/install.sh | bash
```

Or download and inspect first:

```bash
curl -fsSL https://repo.msaeedsaeedi.com/clean-repo/install.sh -o install.sh
bash install.sh
```

### Option 2 — Manual APT Installation

1. Install the GPG Public Key
   ```bash
   curl -fsSL https://repo.msaeedsaeedi.com/clean-repo/key.asc | sudo gpg --dearmor -o /usr/share/keyrings/clean-repo-archive-keyring.gpg
   ```

2. Add the APT repository:

   ```bash
   echo "deb [arch=all signed-by=/usr/share/keyrings/clean-repo-archive-keyring.gpg] https://repo.msaeedsaeedi.com/clean-repo stable main" | \
   sudo tee /etc/apt/sources.list.d/clean-repo.list > /dev/null
   ```

3. Update and install:

   ```bash
   sudo apt update
   sudo apt install clean-repo
   ```

### Option 3 — Manual .deb Installation

1. Download the `.deb` package from the Releases page.
2. Install the package:

   ```bash
   sudo dpkg -i clean-repo.deb
   sudo apt --fix-broken install
   ```

---

## 🔧 Usage

* Preview what would be deleted (dry-run):

  ```bash
  clean-repo
  ```

* Actually delete ignored files:

  ```bash
  clean-repo -x
  ```

* Delete files but keep logs:

  ```bash
  clean-repo -i "*.log" -x
  ```

* Get detailed help:

  ```bash
  clean-repo --help
  man clean-repo
  ```

---

## 📝 License

MIT License. Feel free to use, modify, and distribute.

---

## 💬 Feedback & Contributions

Issues and PRs are welcome. If you have suggestions or feature requests, feel free to open an issue.
