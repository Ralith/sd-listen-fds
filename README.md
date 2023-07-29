# sd-listen-fds

[![Documentation](https://docs.rs/oddio/badge.svg)](https://docs.rs/sd-listen-fds/)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)

Exposes file descriptors passed in by systemd, the Linux init daemon,
without dependencies, foreign or otherwise. Enables easy
implementation of socket-activated services in pure Rust.

Unlike services that open sockets themselves, socket-activated
services are started on-demand, may start up concurrently with their
dependencies, and may be restarted without losing inbound
data. Portable applications can call `sd_listen_fds::get()` and open
their own socket if it succeeds with an empty `Vec`.

See also the
[sd_listen_fds](https://www.freedesktop.org/software/systemd/man/sd_listen_fds.html)
API exposed by the foreign `libsystemd` library. Compared to using
`libsystemd` bindings, this crate is smaller, safer, and builds
everywhere.

### Example

```rust
let fds = sd_listen_fds::get().unwrap();
let (_name, fd) = fds
    .into_iter()
    .next()
    .expect("must be launched as a systemd socket-activated service");
let socket = TcpListener::from(fd);
```

my-service.socket (see also [systemd.socket](https://www.freedesktop.org/software/systemd/man/systemd.socket.html))
```ini
[Socket]
# TCP port number. Other types of sockets are also possible.
ListenStream=1234

[Install]
WantedBy=sockets.target

[Unit]
Description=My Rust service
Documentation=https://example.com/my-service/
```

my-service.service (see also [systemd.service](https://www.freedesktop.org/software/systemd/man/systemd.service.html))
```ini
[Service]
ExecStart=/path/to/my-service

[Unit]
Requires=my-service.socket
Description=My Rust service
Documentation=https://example.com/my-service/
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
