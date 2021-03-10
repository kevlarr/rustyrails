use std::future::Future;

use actix_files::Files;
use actix_service::ServiceFactory;
use actix_web::{
    dev::{
        Factory,
        RequestHead,
        ServiceRequest,
        ServiceResponse,
    },
    guard::fn_guard,
    web,
    FromRequest,
    Responder,
    Scope,
};


pub struct Router {
    scope: Scope,
}

impl Router {
    pub fn new(path: &str) -> Self {
        Self {
            scope: web::scope(path),
        }
    }

    pub fn service(self) -> Scope {
        self.scope
    }
}

/// Wrappers for typical Actix Web routing strategies
impl Router {
    pub fn fallback<F, T, R, U>(mut self, handler: F) -> Self
    where
        F: Factory<T, R, U>,
        T: FromRequest + 'static,
        R: Future<Output = U> + 'static,
        U: Responder + 'static,
    {
        self.scope = self.scope.default_service(web::route().to(handler));
        self
    }

    pub fn scope(mut self, path: &str, cb: impl Fn(Self) -> Self) -> Self {
        let routes = Self { scope: web::scope(path) };

        self.scope = self.scope.service(cb(routes).scope);
        self
    }

    pub fn guard<F>(mut self, f: F) -> Self
    where
        F: Fn(&RequestHead) -> bool + 'static,
    {
        self.scope = self.scope.guard(fn_guard(f));
        self
    }

    pub fn map_files<F>(
        mut self,
        urlpath: &str,
        dirpath: &str,
        cb: impl Fn(Scope) -> Scope<F>
    ) -> Self
    where
        F: ServiceFactory<
            Config = (),
            Request = ServiceRequest,
            Response = ServiceResponse,
            Error = actix_web::Error,
            InitError = (),
        > + 'static,
    {
        let handler = Files::new("/", dirpath)
            .show_files_listing()
            .use_last_modified(true);

        let scope = web::scope(urlpath).service(handler);

        self.scope = self.scope.service(cb(scope));
        self
    }
}

/// Implementation of supported HTTP request methods
impl Router {
    pub fn delete<F, T, R, U>(mut self, path: &str, handler: F) -> Self
    where
        F: Factory<T, R, U>,
        T: FromRequest + 'static,
        R: Future<Output = U> + 'static,
        U: Responder + 'static,
    {
        self.scope = self.scope.route(path, web::delete().to(handler));
        self
    }

    pub fn get<F, T, R, U>(mut self, path: &str, handler: F) -> Self
    where
        F: Factory<T, R, U>,
        T: FromRequest + 'static,
        R: Future<Output = U> + 'static,
        U: Responder + 'static,
    {
        self.scope = self.scope.route(path, web::get().to(handler));
        self
    }

    pub fn head<F, T, R, U>(mut self, path: &str, handler: F) -> Self
    where
        F: Factory<T, R, U>,
        T: FromRequest + 'static,
        R: Future<Output = U> + 'static,
        U: Responder + 'static,
    {
        self.scope = self.scope.route(path, web::head().to(handler));
        self
    }

    pub fn patch<F, T, R, U>(mut self, path: &str, handler: F) -> Self
    where
        F: Factory<T, R, U>,
        T: FromRequest + 'static,
        R: Future<Output = U> + 'static,
        U: Responder + 'static,
    {
        self.scope = self.scope.route(path, web::patch().to(handler));
        self
    }

    pub fn post<F, T, R, U>(mut self, path: &str, handler: F) -> Self
    where
        F: Factory<T, R, U>,
        T: FromRequest + 'static,
        R: Future<Output = U> + 'static,
        U: Responder + 'static,
    {
        self.scope = self.scope.route(path, web::post().to(handler));
        self
    }

    pub fn put<F, T, R, U>(mut self, path: &str, handler: F) -> Self
    where
        F: Factory<T, R, U>,
        T: FromRequest + 'static,
        R: Future<Output = U> + 'static,
        U: Responder + 'static,
    {
        self.scope = self.scope.route(path, web::put().to(handler));
        self
    }

    pub fn trace<F, T, R, U>(mut self, path: &str, handler: F) -> Self
    where
        F: Factory<T, R, U>,
        T: FromRequest + 'static,
        R: Future<Output = U> + 'static,
        U: Responder + 'static,
    {
        self.scope = self.scope.route(path, web::trace().to(handler));
        self
    }
}
