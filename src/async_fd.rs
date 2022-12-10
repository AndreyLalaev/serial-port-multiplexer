use std::io;
use std::os::unix::prelude::RawFd;

use nix::unistd::{read, write};
use tokio::io::unix::{self, AsyncFd};
use tokio::io::{AsyncRead, AsyncWrite};

pub struct AsyncIOFd {
    inner: unix::AsyncFd<RawFd>,
}

impl AsyncIOFd {
    pub fn new(fd: RawFd) -> io::Result<Self> {
        Ok(Self {
            inner: AsyncFd::new(fd)?,
        })
    }

    pub async fn read(&self, out: &mut [u8]) -> io::Result<usize> {
        loop {
            let mut guard = self.inner.readable().await?;

            match guard
                .try_io(|inner| read(*inner.get_ref(), out).map_err(|errno| io::Error::from(errno)))
            {
                Ok(count) => return count,
                Err(_) => continue,
            }
        }
    }

    pub async fn write(&self, buf: &[u8]) -> io::Result<usize> {
        loop {
            let mut guard = self.inner.writable().await?;

            match guard.try_io(|inner| {
                write(*inner.get_ref(), buf).map_err(|errno| io::Error::from(errno))
            }) {
                Ok(count) => return count,
                Err(_) => continue,
            }
        }
    }
}
