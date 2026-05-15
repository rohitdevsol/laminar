# Contributing To Laminar

Thanks for contributing to Laminar.

---

# Development Workflow

Before pushing code, run:

```bash
make fix
make verify
```

CI will fail if these checks do not pass.

---

# Branching

Do not commit directly to `main`.

Create a feature branch:

```bash
git checkout -b feature/my-change
```

Examples:

```txt
feature/round-robin
fix/backend-timeout
refactor/config-loader
```

---

# Commit Style

Recommended prefixes:

```txt
feat:
fix:
refactor:
perf:
docs:
test:
chore:
infra:
```

Example:

```txt
feat: add round robin balancer
```

---

# Coding Guidelines

- Prefer clarity over clever abstractions
- Avoid premature optimization
- Keep modules focused and maintainable
- Document important architectural decisions

---

# Testing

Run all checks locally:

```bash
make verify
```

Run specific tests:

```bash
cargo test --test ping_check -- --nocapture
```

---

# License

By contributing, you agree that contributions will be licensed under:

- MIT
