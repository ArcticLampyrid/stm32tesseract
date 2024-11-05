use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use reqwest::Url;
use sha2::Digest;
use std::path::PathBuf;
use thiserror::Error;
use trauma::{
    download::{Download, Status, Summary},
    downloader::DownloaderBuilder,
};

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("invalid url")]
    InvalidUrl(#[from] url::ParseError),
    #[error("download fail: {0}")]
    DownloadFail(String),
    #[error("download not started")]
    DownloadNotStarted,
    #[error("failed to communicate with the download manager")]
    DownloadManagerCommunication,
}

enum DownloadRequest {
    Download {
        url: String,
        notify: tokio::sync::oneshot::Sender<Result<Summary, DownloadError>>,
    },
    Quit,
}

struct DownloadManager {
    root: PathBuf,
    tx: tokio::sync::mpsc::UnboundedSender<DownloadRequest>,
    background: Option<std::thread::JoinHandle<()>>,
}

static DOWNLOAD_MANAGER: once_cell::sync::Lazy<DownloadManager> =
    once_cell::sync::Lazy::new(|| {
        let root = std::env::temp_dir()
            .join("stm32tesseract")
            .join("downloads");
        let _ = std::fs::create_dir_all(&root);
        DownloadManager::new(root)
    });

fn auto_file_name(url: &str) -> String {
    let filename = url.split('/').last().unwrap();
    let filename = filename.split('?').next().unwrap();
    let filename = filename.replace(
        |c: char| !(c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.'),
        "_",
    );
    let mut full = {
        let mut hasher = sha2::Sha256::new();
        hasher.update(filename.as_bytes());
        let result = hasher.finalize();
        URL_SAFE_NO_PAD.encode(result.as_slice())
    };
    full.push_str(&filename);
    full
}

pub fn download_file(url: &str) -> Result<PathBuf, DownloadError> {
    DOWNLOAD_MANAGER.download(url)
}

impl DownloadManager {
    pub fn new(root: PathBuf) -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let background = std::thread::spawn({
            let root = root.clone();
            move || {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(Self::background_task(root, rx));
            }
        });
        Self {
            root,
            tx,
            background: Some(background),
        }
    }

    async fn background_task(
        root: PathBuf,
        mut rx: tokio::sync::mpsc::UnboundedReceiver<DownloadRequest>,
    ) {
        let downloader = DownloaderBuilder::new().directory(root).retries(5).build();
        while let Some(req) = rx.recv().await {
            match req {
                DownloadRequest::Download { url, notify } => {
                    let filename = auto_file_name(url.as_str());
                    match Url::parse(url.as_str()) {
                        Ok(url) => {
                            let info = Download::new(&url, &filename);
                            let list = vec![info];
                            let result = downloader.download(list.as_slice()).await;
                            let _ = notify.send(Ok(result.into_iter().next().unwrap()));
                        }
                        Err(e) => {
                            let _ = notify.send(Err(e.into()));
                        }
                    }
                }
                DownloadRequest::Quit => {
                    break;
                }
            }
        }
    }

    pub fn download(&self, url: &str) -> Result<PathBuf, DownloadError> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.tx
            .send(DownloadRequest::Download {
                url: url.to_string(),
                notify: tx,
            })
            .unwrap();

        let summary = rx
            .blocking_recv()
            .map_err(|_| DownloadError::DownloadManagerCommunication)??;
        match summary.status() {
            Status::Fail(reason) => Err(DownloadError::DownloadFail(reason.clone())),
            Status::NotStarted => Err(DownloadError::DownloadNotStarted),
            Status::Skipped(_) | Status::Success => {
                let filename = summary.download().filename.as_str();
                Ok(self.root.join(filename))
            }
        }
    }
}

impl Drop for DownloadManager {
    fn drop(&mut self) {
        let background = self.background.take();
        if let Some(handle) = background {
            self.tx.send(DownloadRequest::Quit).unwrap();
            handle.join().unwrap();
        }
    }
}
