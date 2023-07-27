use std::{env, fmt, mem, os::fd::OwnedFd};

/// Fetch any sockets provided by systemd, and their names if specified
///
/// See [sd_listen_fds](https://www.freedesktop.org/software/systemd/man/sd_listen_fds.html).
///
/// Succeeds with an empty `Vec` if no socket was provided, e.g. if not started by systemd,
/// including on non-Linux platforms.
#[cfg_attr(not(target_os = "linux"), inline)]
pub fn get() -> Result<Vec<(Option<String>, OwnedFd)>, Error> {
    #[cfg(target_os = "linux")]
    {
        let Ok(pid) = env::var("LISTEN_PID") else { return Ok(Vec::new()); };
        let pid = pid.parse::<u32>().map_err(|_| Error::MalformedEnv)?;
        if pid != std::process::id() {
            return Ok(Vec::new());
        }
        let Ok(fd_count) = env::var("LISTEN_FDS") else { return Ok(Vec::new()); };
        let fd_count = fd_count.parse::<u32>().map_err(|_| Error::MalformedEnv)?;

        const SD_LISTEN_FDS_START: u32 = 3;
        if fd_count > u32::MAX - 3 {
            return Err(Error::MalformedEnv);
        }

        let names_raw = env::var("LISTEN_FDNAMES").unwrap_or_else(|_| String::new());
        let mut names = names_raw.split(':');
        let mut result = Vec::with_capacity(fd_count as usize);
        for i in 0..fd_count {
            let name = match names_raw.is_empty() {
                false => names.next().map(str::to_owned),
                true => None,
            };
            let fd = unsafe { mem::transmute::<u32, OwnedFd>(SD_LISTEN_FDS_START + i) };
            result.push((name, fd));
        }

        Ok(result)
    }
    #[cfg(not(target_os = "linux"))]
    {
        Ok(Vec::new())
    }
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Error {
    /// The environment contained non-empty, malformed socket activation variables (`LISTEN_PID`
    /// and/or `LISTEN_FDS`)
    MalformedEnv,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("malformed socket activation environment")
    }
}
