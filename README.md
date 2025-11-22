# SplitDNS

A simple, cross-platform Rust library for DNS split-domain configuration.
Provides minimal but functional DNS split-tunneling similar to Tailscale's
approach.

## Platforms

- [x] macOS using resolver files in `/etc/resolver/<domain>`
- [x] Linux using resolve1 dbus api
- [ ] Windows using PowerShell to configure NRPT rules

More platforms will be added as I test them.

## Platform Support

### macOS

**Generated resolver file:**

```bash
# /etc/resolver/internal.example.com
domain internal.example.com
nameserver 192.168.1.10
port 53
```

## License

MIT
