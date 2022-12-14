use std::{fmt::Display, fs::File, io::read_to_string};

use anyhow::{Context, Result};
use inotify::WatchMask;
use tar::Archive;

use crate::{peer_conf::PeerConfig, zone::Zone};

use super::{file_watcher::FILE_WATCHER, Reader};

#[derive(Debug)]
pub struct TarReader {
    zone: &'static Zone,
}

impl TarReader {
    pub fn new(zone: &'static Zone) -> TarReader {
        TarReader { zone }
    }
}

impl Reader for TarReader {
    fn get_zone(&self) -> &'static Zone {
        self.zone
    }

    fn collect(&self) -> Result<Vec<PeerConfig>> {
        Archive::new(File::open(&self.zone.path).context("open tar file")?)
            .entries()
            .context("list files in tar")?
            .try_fold(Vec::new(), |mut peers, peer_file| {
                peers.push(PeerConfig::from(
                    read_to_string(peer_file.context("resolve tar entry")?)
                        .context("read tar entry to str")?,
                )?);
                Ok(peers)
            })
    }

    fn start_watching(&mut self) -> Result<()> {
        FILE_WATCHER
            .lock()?
            .add_watch(&self.zone.path, WatchMask::CLOSE_WRITE)?;
        Ok(())
    }
}

impl Display for TarReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "tar({:?})", self.zone)
    }
}
