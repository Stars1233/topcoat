use std::{
    collections::HashSet,
    task::{Context, Poll},
};

use http::{Request, Uri, uri::PathAndQuery};
use tower_http::services::{ServeDir, fs::DefaultServeDirFallback};
use tower_service::Service;

use crate::AssetBundle;

/// `tower` service that serves the files in an [`AssetBundle`] over HTTP.
///
/// Only filenames present in the bundle are served; any other path
/// receives a 404 (or is forwarded to the configured fallback, if any).
#[derive(Clone, Debug)]
pub struct ServeAssetBundle<F = DefaultServeDirFallback> {
    inner: ServeDir<F>,
    files: HashSet<String>,
}

impl ServeAssetBundle {
    /// Build a service that serves `bundle`.
    pub fn new(bundle: &AssetBundle) -> Self {
        let files = bundle
            .assets()
            .filter_map(|asset| asset.path().file_name()?.to_str().map(String::from))
            .collect();
        Self {
            inner: ServeDir::new(bundle.dir()),
            files,
        }
    }
}

impl<F> ServeAssetBundle<F> {
    /// Set a fallback service to handle requests for paths that aren't
    /// part of the bundle.
    pub fn fallback<F2>(self, fallback: F2) -> ServeAssetBundle<F2> {
        ServeAssetBundle {
            inner: self.inner.fallback(fallback),
            files: self.files,
        }
    }
}

impl<B, F> Service<Request<B>> for ServeAssetBundle<F>
where
    ServeDir<F>: Service<Request<B>>,
{
    type Response = <ServeDir<F> as Service<Request<B>>>::Response;
    type Error = <ServeDir<F> as Service<Request<B>>>::Error;
    type Future = <ServeDir<F> as Service<Request<B>>>::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        let path = req.uri().path().trim_start_matches('/');

        if !self.files.contains(path) {
            // Force ServeDir's invalid-path branch (404 or fallback) without
            // hitting the disk by replacing the path with a parent traversal.
            let mut parts = req.uri().clone().into_parts();
            parts.path_and_query = Some(PathAndQuery::from_static("/.."));
            *req.uri_mut() = Uri::from_parts(parts).unwrap();
        }
        self.inner.call(req)
    }
}
