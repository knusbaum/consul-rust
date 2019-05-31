use bytes::Bytes;
use hyper;
use hyper::rt::{Future, Stream};
use hyper::{Client, Body, Response};
use hyper::client::HttpConnector;
use hyper::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use hyper::Request;
use hyper_openssl::HttpsConnector;
use std::io::Read;
use error::ConsulResult;
use std::string::String;

type Error = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct Handler {
    client: Client<HttpsConnector<HttpConnector>, Body>,
    url: String
}

impl Handler {
    pub fn new(url: &str) -> Handler {
        let connector = HttpsConnector::new(1).unwrap();
        let client = Client::builder().build(connector);
        
        Handler {
            client: client,
            url: url.to_owned()
        }
    }

    fn read_to_string(resp: Response<Body>) -> Result<String, String> {
        let body = resp.into_body();
//        body.map(|chunk| chunk.into_bytes().into())
//            .fold("".to_string(), |acc, x: String| { acc.push_str(&x); acc })

        body.map_err(Error::from)
            .concat2()
            .and_then(|c| {
                std::str::from_utf8(&c).map(str::to_owned).map_err(Error::from)
            })
            .map_err(|e| format!("Error: {}", e))
            .wait()

    }

    pub fn get(&self, endpoint: &str) -> ConsulResult<String> {
        let full_url = format!("{}/{}", self.url.trim_right_matches('/'), endpoint);
        let mut res = self.client.get(full_url.parse().unwrap())
            .wait();

        match res {
            Ok(resp) => {
                Handler::read_to_string(resp)
            },
            Err(e) => {
                Err(e.to_string())
            }
        }

    }

    pub fn _post(&self, endpoint: &str, req: String) -> ConsulResult<String> {
        let full_url = format!("{}/{}", self.url.trim_right_matches('/'), endpoint);

        let req = Request::builder()
            .method("POST")
            .uri(&full_url)
            .body(Body::from(req))
            .map_err(|e| e.to_string())?;

        let mut res = self.client.request(req).wait();
        match res {
            Ok(req) => {
                Handler::read_to_string(req)
            },
            Err(e) => {
                Err(e.to_string())
            }
        }
    }

    pub fn put(&self, endpoint: &str, req: String, content_type: Option<&str>) -> ConsulResult<String> {
        let full_uri = format!("{}/{}", self.url.trim_right_matches('/'), endpoint);


        let mut request; 
        
        if let Some(content) = content_type {
            let mime = HeaderValue::from_str(content).unwrap();
            //let mut headers = HeaderMap::new();
            //headers.insert(CONTENT_TYPE, mime);

            //request = request.headers(headers);
            request = Request::builder()
                .method("PUT")
                .uri(&full_uri)
                .header(CONTENT_TYPE, mime)
                .body(Body::from(req))
                .map_err(|e| e.to_string())?;
            
//            res = self.client.put(&full_url)
//                .headers(headers)
//                .body(&req)
//                .send()
//                .map_err(|e| format!("{}", e))?;

        }
        else {
            request = Request::builder()
                .method("PUT")
                .uri(&full_uri)
                .body(Body::from(req))
                .map_err(|e| e.to_string())?;
        }
//        else {
//            res = self.client.put(&full_url)
//                .body(&req)
//                .send()
//                .map_err(|e| format!("{}", e))?;
//        }

//        let req = request
//            .body(Body::from(req))
//            .map_err(|e| e.to_string())?;
        
//        if res.status == hyper::StatusCode::OK {
//            let mut response = String::new();
//            res.read_to_string(&mut response)
//                .map_err(|e| format!("{}", e))?;
//            Ok(response)
//        }
//        else {
//            let mut response = String::new();
//            res.read_to_string(&mut response)
//                .map_err(|e| format!("{}", e))?;
//
//            if !response.is_empty() {
//                Ok(response)
//            }
//            else {
//                Err(format!("Request failed with status: {:?}", res.status_raw()))
//            }
//        }

        let mut res = self.client.request(request).wait();
        match res {
            Ok(req) => {
                Handler::read_to_string(req)
            },
            Err(e) => {
                Err(e.to_string())
            }
        }
    }

    pub fn delete(&self, endpoint: &str) -> ConsulResult<String> {
        let full_url = format!("{}/{}", self.url.trim_right_matches('/'), endpoint);

        let req = Request::builder()
            .method("DELETE")
            .uri(&full_url)
            .body(Body::empty())
            .map_err(|e| e.to_string())?;

        let mut res = self.client.request(req).wait();
        match res {
            Ok(req) => {
                Handler::read_to_string(req)
            },
            Err(e) => {
                Err(e.to_string())
            }
        }

//        let mut res = self.client.delete(&full_url)
//            .send()
//            .map_err(|e| format!("{}", e))?;
//
//        if res.status == hyper::StatusCode::OK {
//            let mut response = String::new();
//            res.read_to_string(&mut response)
//                .map_err(|e| format!("{}", e))?;
//            Ok(response)
//        }
//        else {
//            let mut response = String::new();
//            res.read_to_string(&mut response)
//                .map_err(|e| format!("{}", e))?;
//
//            if !response.is_empty() {
//                Ok(response)
//            }
//            else {
//                Err(format!("Request failed with status: {:?}", res.status_raw()))
//            }
//        }
    }


}
