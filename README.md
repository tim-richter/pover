# pover
[![GitHub current release](https://img.shields.io/github/v/release/tim-richter/pover?include_prereleases)](https://github.com/tim-richter/pover/releases/latest)

Utility script that makes working with package.json files easier.

## Installation

### MacOS

- Download macos binary from releases
- Move binary to /usr/local/bin

  ```bash
  sudo mv pover-macos-amd64 /usr/local/bin/pover
  ```
- Make executable

  ```bash
  chmod +x /usr/local/bin/pover
  ```
  
### Linux

- Download linux binary from releases
- Move binary to /usr/local/bin

  ```bash
  sudo mv pover-linux-amd64 /usr/local/bin/pover
  ```
- Make executable

  ```bash
  chmod +x /usr/local/bin/pover
  ```

## Usage

pover tries to find the package.json in your current working directory so make sure that you are in your projects root folder.

---

**Display all scripts defined in package.json**

```bash
pover
```

**Open the project in the browser (uses "respository" in your package.json)**

```bash
pover repo
```
