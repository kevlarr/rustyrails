use std::future::Future;

//use actix_files::Files;
//use actix_service::ServiceFactory;
use actix_web::{
    dev::{
        Factory,
        //RequestHead,
        //ServiceRequest,
        //ServiceResponse,
    },
    //guard::fn_guard,
    web,
    FromRequest,
    Responder,
    Scope,
};



pub struct Resource<T>
where
    T: FromRequest + 'static
{
    scope: Scope,
    _marker: std::marker::PhantomData<&'static T>,
}

impl<T> Resource<T>
where
    T: FromRequest + 'static
{
    pub fn new() -> Self {
        Self {
            scope: web::scope(""),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn get<F, T1, R, U>(mut self, path: &str, handler: F) -> Self
    where
        F: Factory<T1, R, U>,
        R: Future<Output = U> + 'static,
        U: Responder + 'static,
        T1: FromRequest + 'static,
    {
        self.scope = self.scope.route(path, web::get().to(handler));
        self
    }
}


pub struct Router {
    scope: Scope,
}

impl Router {
    //pub fn new(path: &str) -> Self {
    pub fn new() -> Self {
        Self {
            scope: web::scope(""),
        }
    }

    pub fn routes(self) -> Scope {
        self.scope
    }

    //pub fn default<F, T, R, U>(mut self, handler: F) -> Self
    //where
        //F: Factory<T, R, U>,
        //T: FromRequest + 'static,
        //R: Future<Output = U> + 'static,
        //U: Responder + 'static,
    //{
        //self.service = self.service.default_service(web::route().to(handler));
        //self
    //}

    pub fn resource<T>(mut self, path: &str, res: Resource<T>) -> Self
    where
        T: FromRequest + 'static,
    {
        self.scope = self.scope.service(
            web::scope(path).service(res.scope)
        );
        self
    }

    //pub fn guard<F>(mut self, f: F) -> Self
    //where
        //F: Fn(&RequestHead) -> bool + 'static,
    //{
        //self.service = self.service.guard(fn_guard(f));
        //self
    //}

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

    //pub fn post<F, T, R, U>(mut self, path: &str, handler: F) -> Self
    //where
        //F: Factory<T, R, U>,
        //T: FromRequest + 'static,
        //R: Future<Output = U> + 'static,
        //U: Responder + 'static,
    //{
        //self.service = self.service.route(path, web::post().to(handler));
        //self
    //}

    //pub fn patch<F, T, R, U>(mut self, path: &str, handler: F) -> Self
    //where
        //F: Factory<T, R, U>,
        //T: FromRequest + 'static,
        //R: Future<Output = U> + 'static,
        //U: Responder + 'static,
    //{
        //self.service = self.service.route(path, web::patch().to(handler));
        //self
    //}

    //pub fn map_files<F>(
        //mut self,
        //urlpath: &str,
        //dirpath: &str,
        //cb: impl Fn(Scope) -> Scope<F>
    //) -> Self
    //where
        //F: ServiceFactory<
            //Config = (),
            //Request = ServiceRequest,
            //Response = ServiceResponse,
            //Error = actix_web::Error,
            //InitError = (),
        //> + 'static,
    //{
        //let handler = Files::new("/", dirpath)
            //.show_files_listing()
            //.use_last_modified(true);

        //let scope = web::scope(urlpath).service(handler);

        //self.service = self.service.service(cb(scope));
        //self
    //}
}
