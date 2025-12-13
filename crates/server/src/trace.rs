//! Wire tracing for debugging MCP handshakes.
//!
//! It wraps stdio transport with optional
//! wire tracing, mirroring traffic to stderr in hex+UTF8 for debugging
//! Machine-Readable Context Protocol (MCP) handshakes.

use base64::Engine;
use rmcp::transport;
use tokio::io::{AsyncRead, AsyncWrite};

/// Wraps stdio transport with optional wire tracing for debugging.
pub fn stdio_with_optional_trace(trace: bool) -> (StdioReader, StdioWriter) {
    let (stdin, stdout) = transport::stdio();
    if !trace {
        return (Box::pin(stdin), Box::pin(stdout));
    }

    (
        Box::pin(LoggingReader {
            inner: stdin,
            label: "in",
        }),
        Box::pin(LoggingWriter {
            inner: stdout,
            label: "out",
        }),
    )
}

pub type StdioReader = Pin<Box<dyn AsyncRead + Unpin + Send + 'static>>;
pub type StdioWriter = Pin<Box<dyn AsyncWrite + Unpin + Send + 'static>>;

use std::pin::Pin;

/// Reader wrapper that mirrors traffic to stderr in hex+UTF8 for debugging.
pub struct LoggingReader<R> {
    pub inner: R,
    pub label: &'static str,
}

impl<R: AsyncRead + Unpin> AsyncRead for LoggingReader<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let pre = buf.filled().len();
        let poll = Pin::new(&mut self.inner).poll_read(cx, buf);
        if let std::task::Poll::Ready(Ok(())) = &poll {
            let read = &buf.filled()[pre..];
            if !read.is_empty() {
                eprintln!(
                    "[wire {}] {} bytes: {} | {}",
                    self.label,
                    read.len(),
                    base64::engine::general_purpose::STANDARD.encode(read),
                    String::from_utf8_lossy(read)
                );
            }
        }
        poll
    }
}

/// Writer wrapper that mirrors traffic to stderr in hex+UTF8 for debugging.
pub struct LoggingWriter<W> {
    pub inner: W,
    pub label: &'static str,
}

impl<W: AsyncWrite + Unpin> AsyncWrite for LoggingWriter<W> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        if !buf.is_empty() {
            eprintln!(
                "[wire {}] {} bytes: {} | {}",
                self.label,
                buf.len(),
                base64::engine::general_purpose::STANDARD.encode(buf),
                String::from_utf8_lossy(buf)
            );
        }
        Pin::new(&mut self.inner).poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }
}
