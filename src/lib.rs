extern crate rand;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate bytes;
extern crate byteorder;

use std::mem;
use std::io;

use byteorder::{NativeEndian};
use bytes::{BytesMut, BufMut, IntoBuf, Buf};
use tokio_io::codec::{Encoder, Decoder};

pub struct Codec;

impl Decoder for Codec {
    type Item = u64;
    type Error = io::Error;


    fn decode(&mut self, buff: &mut BytesMut) -> io::Result<Option<Self::Item>> {
        //While you can get u64 regardless of current len, it cannot be complete unless we have
        //enough bytes
        if buff.len() >= mem::size_of::<u64>() {
            let result = buff.split_to(mem::size_of::<u64>());
            let mut result = result.into_buf();
            let result = result.get_u64::<NativeEndian>();
            Ok(Some(result))
        }
        else {
            Ok(None)
        }
    }
}

use std::net;
use rand::{Rng};

impl Encoder for Codec {
    type Item = u64;
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buff: &mut BytesMut) -> io::Result<()> {
        buff.reserve(msg as usize * 40);
        //For simplicity sake separate by \n
        buff.put_u64::<NativeEndian>(msg);
        buff.put("\n");

        let mut random = rand::thread_rng();
        for _ in 0..msg {
            let ip = net::IpAddr::V4(net::Ipv4Addr::new(random.gen(), random.gen(), random.gen(), random.gen()));
            let addr = net::SocketAddr::new(ip, random.gen());
            buff.put(&format!("{}\n", addr));
        }

        Ok(())
    }
}

use tokio_proto::pipeline::ServerProto;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;

pub struct Server;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for Server {
    type Request = u64;

    /// For this protocol style, `Response` matches the coded `Out` type
    type Response = u64;

    /// A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, Codec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(Codec))
    }
}

use tokio_service::Service;
use futures::{future, Future, BoxFuture};

pub struct ServeAddrs;

impl Service for ServeAddrs {
    // These types must match the corresponding protocol types:
    type Request = u64;
    type Response = u64;

    type Error = io::Error;

    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        future::ok(req).boxed()
    }
}
