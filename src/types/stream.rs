use bytes::{BufMut, Bytes, BytesMut};
use futures::{future, stream, Stream, StreamExt};
use hyper::Body;
use pin_project::pin_project;
use std::{
    fmt,
    io::{self, Error as IoError},
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::{AsyncRead, ReadBuf};

/// Stream of bytes.
#[pin_project]
pub struct ByteStream {
    size_hint: (usize, Option<usize>),
    #[pin]
    inner: Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send + 'static>>,
}
impl ByteStream {
    /// Create a new `ByteStream` by wrapping a `futures` stream.
    pub fn new<S>(stream: S) -> ByteStream
    where
        S: Stream<Item = Result<Bytes, io::Error>> + Send + 'static,
    {
        ByteStream {
            size_hint: stream.size_hint(),
            inner: Box::pin(stream),
        }
    }

    /// Creates a new `ByteStream` by wrapping a `futures` stream. Allows for the additional size_hint.
    pub fn new_with_size<S>(stream: S, size_hint: usize) -> ByteStream
    where
        S: Stream<Item = Result<Bytes, io::Error>> + Send + 'static,
    {
        ByteStream {
            size_hint: (0, Some(size_hint)),
            inner: Box::pin(stream),
        }
    }

    pub(crate) fn size_hint(&self) -> (usize, Option<usize>) {
        self.size_hint
    }

    /// Return an implementation of `AsyncRead` that uses async i/o to consume the stream.
    pub fn into_async_read(self) -> impl AsyncRead + Send {
        ImplAsyncRead::new(self.inner)
    }

    /// Return an implementation of `Read` that uses blocking i/o to consume the stream.
    pub fn into_blocking_read(self) -> impl io::Read + Send {
        ImplBlockingRead::new(self.inner)
    }
}

impl From<Vec<u8>> for ByteStream {
    fn from(buf: Vec<u8>) -> ByteStream {
        ByteStream {
            size_hint: (buf.len(), Some(buf.len())),
            inner: Box::pin(stream::once(async move { Ok(Bytes::from(buf)) })),
        }
    }
}
impl From<Body> for ByteStream {
    fn from(body: Body) -> Self {
        ByteStream::new(body.map(|try_chunk| {
            try_chunk.map(|c| c).map_err(|e| {
                IoError::new(
                    io::ErrorKind::Other,
                    format!("Error obtaining chunk: {}", e),
                )
            })
        }))
    }
}

impl fmt::Debug for ByteStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<ByteStream size_hint={:?}>", self.size_hint)
    }
}

impl Stream for ByteStream {
    type Item = Result<Bytes, io::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        this.inner.poll_next(cx)
    }
}

#[pin_project]
struct ImplAsyncRead {
    buffer: BytesMut,
    #[pin]
    stream: futures::stream::Fuse<Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send>>>,
}

impl ImplAsyncRead {
    fn new(stream: Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send>>) -> Self {
        ImplAsyncRead {
            buffer: BytesMut::new(),
            stream: stream.fuse(),
        }
    }
}

impl AsyncRead for ImplAsyncRead {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf,
    ) -> Poll<io::Result<()>> {
        let this = self.project();
        if this.buffer.is_empty() {
            match futures::ready!(this.stream.poll_next(cx)) {
                None => return Poll::Ready(Ok(())),
                Some(Err(e)) => return Poll::Ready(Err(e)),
                Some(Ok(bytes)) => {
                    this.buffer.put(bytes);
                }
            }
        }
        let available = std::cmp::min(buf.remaining(), this.buffer.len());
        let bytes = this.buffer.split_to(available);
        buf.put_slice(&bytes);
        Poll::Ready(Ok(()))
    }
}

#[pin_project]
struct ImplBlockingRead {
    #[pin]
    inner: ImplAsyncRead,
}

impl ImplBlockingRead {
    fn new(stream: Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send>>) -> Self {
        ImplBlockingRead {
            inner: ImplAsyncRead::new(stream),
        }
    }
}

impl io::Read for ImplBlockingRead {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(future::poll_fn(|cx| {
            let mut buf = ReadBuf::new(buf);
            futures::ready!(AsyncRead::poll_read(
                Pin::new(&mut self.inner),
                cx,
                &mut buf
            ))?;
            Poll::Ready(Ok(buf.filled().len()))
        }))
    }
}

#[tokio::test]
async fn test_async_read() {
    use bytes::Bytes;
    use tokio::io::AsyncReadExt;

    let chunks = vec![
        Ok(Bytes::from_static(b"Shimo")),
        Ok(Bytes::from_static(b"Doc")),
    ];
    let stream = ByteStream::new(stream::iter(chunks));
    let mut async_read = stream.into_async_read();

    // Buffer with capacity of 3
    let mut buf = [0u8; 3];
    assert_eq!(async_read.read(&mut buf).await.unwrap(), 3);
    assert_eq!(&buf[..3], b"Shi");
    assert_eq!(async_read.read(&mut buf).await.unwrap(), 2);
    assert_eq!(&buf[..2], b"mo");
    assert_eq!(async_read.read(&mut buf).await.unwrap(), 3);
    assert_eq!(&buf[..3], b"Doc");
    assert_eq!(async_read.read(&mut buf).await.unwrap(), 0);
}

#[test]
fn test_blocking_read() {
    use bytes::Bytes;
    use std::io::Read;

    let chunks = vec![
        Ok(Bytes::from_static(b"Shimo")),
        Ok(Bytes::from_static(b"Doc")),
    ];
    let stream = ByteStream::new(stream::iter(chunks));
    let mut block_read = stream.into_blocking_read();

    // Buffer with capacity of 3
    let mut buf = [0u8; 3];
    assert_eq!(block_read.read(&mut buf).unwrap(), 3);
    assert_eq!(&buf[..3], b"Shi");
    assert_eq!(block_read.read(&mut buf).unwrap(), 2);
    assert_eq!(&buf[..2], b"mo");
    assert_eq!(block_read.read(&mut buf).unwrap(), 3);
    assert_eq!(&buf[..3], b"Doc");
    assert_eq!(block_read.read(&mut buf).unwrap(), 0);
}

#[tokio::test]
async fn test_new_with_size_read() {
    use bytes::Bytes;
    use tokio::io::AsyncReadExt;

    let chunks = vec![
        Ok(Bytes::from_static(b"Shimo")),
        Ok(Bytes::from_static(b"Doc")),
    ];
    let stream = ByteStream::new_with_size(stream::iter(chunks), 8);

    assert_eq!(stream.size_hint, (0, Some(8)));

    let mut async_read = stream.into_async_read();

    let mut buf = [0u8; 3];
    assert_eq!(async_read.read(&mut buf).await.unwrap(), 3);
    assert_eq!(&buf[..3], b"Shi");
    assert_eq!(async_read.read(&mut buf).await.unwrap(), 2);
    assert_eq!(&buf[..2], b"mo");
    assert_eq!(async_read.read(&mut buf).await.unwrap(), 3);
    assert_eq!(&buf[..3], b"Doc");
    assert_eq!(async_read.read(&mut buf).await.unwrap(), 0);
}
