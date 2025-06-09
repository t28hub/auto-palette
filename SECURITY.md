# Security Policy

`auto-palette` is committed to protecting the security of its codebase, CLI, and WebAssembly crates.  
This policy explains **which versions receive fixes**, **how to report vulnerabilities**, and **what you can expect from us**.

---

## Supported Versions

Only the latest stable release receives security updates.  
Earlier versions are considered end-of-life.

| Version | Supported | Notes                                |
|---------|-----------|--------------------------------------|
| 0.9.x   | ✅ Yes     | Latest stable release                |
| < 0.9   | ❌ No      | Please upgrade to the latest version |

## Reporting a Vulnerability

If you discover a security vulnerability, please follow these steps:

1. **Keep it private first.** Do not publicly disclose the vulnerability until we have released a patched version.
   * Open a [GitHub security advisory](https://github.com/t28hub/auto-palette/security/advisories/new).
   * **Do not** create a public issue or pull request.
2. Provide detailed information about the vulnerability, including:
   * A clear description of the vulnerability
   * Steps to reproduce the issue
   * Any relevant code snippets or examples
   * The version of `auto-palette` you are using
   * Proposed fixes or workarounds (optional)

## Disclosure Timeline

| Phase                        | Target timeframe       | Notes                                                               |
|------------------------------|------------------------|---------------------------------------------------------------------|
| Acknowledgement              | ≤ 72 hours             | Confirm receipt of your report                                      |
| Investigation                | ≤ 7 days               | Analyze the vulnerability and determine impact                      |
| Patch release (critical)     | ≤ 14 days              | Release a fix for critical vulnerabilities                          |
| Patch release (non-critical) | N/A                    | Non-critical issues will be addressed in the next release cycle     |
| Public advisory              | ≤ 24 hours after patch | Publish a public advisory with details of the vulnerability and fix |

## Scope

* This policy covers the Rust crate, CLI, and WebAssembly packages in **this repository**.
* Vulnerabilities in third-party dependencies should also be reported upstream.