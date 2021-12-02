use tonic::codec::ProstCodec;

mod protocodec {
    pub use tonic::codec::ProstCodec;
}

mod jsoncodec {
    use prost::bytes::{Buf, BufMut};

    struct JsonEncoder<T>(PhantomData<T>);

    impl<T: serde::Serialize> Encoder for JsonEncoder<T> {
        type Item = T;
        type Error = Status;

        fn encode(&mut self, item: Self::Item, buf: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
            let s = serde_json::to_string(&item).unwrap();
            buf.put(s.as_bytes());

            Ok(())
        }
    }

    pub struct JsonDecoder<U>(PhantomData<U>);

    impl<U: for<'a> serde::Deserialize<'a>> Decoder for JsonDecoder<U> {
        type Item = U;
        type Error = Status;

        fn decode(&mut self, buf: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
            let item = match serde_json::from_reader(buf.reader()) {
                Ok(i) => i,
                Err(e) => {
                    return Err(Status::new(tonic::Code::Internal, e.to_string()));
                }
            };

            Ok(item)
        }
    }

    #[derive(Debug, Clone)]
    struct JsonCodec<T, U> {
        _pd: PhantomData<(T, U)>,
    }

    impl<T, U> Default for JsonCodec<T, U> {
        fn default() -> Self {
            Self { _pd: PhantomData }
        }
    }

    impl<T, U> Codec for JsonCodec<T, U>
        where
            T: serde::Serialize + Send + 'static,
            U: for<'a> serde::Deserialize<'a> + Send + Default + 'static,
    {
        type Encode = T;
        type Decode = U;
        type Encoder = JsonEncoder<T>;
        type Decoder = JsonDecoder<U>;

        fn encoder(&mut self) -> Self::Encoder {
            JsonEncoder(PhantomData)
        }

        fn decoder(&mut self) -> Self::Decoder {
            JsonDecoder(PhantomData)
        }
    }

}