use super::*;
use ark_std::format;


/// Scale `Input` error wrapped for passage through Arkworks' `CanonicalDeserialize`
#[derive(Clone,Debug)]
#[repr(transparent)]
pub struct ArkScaleError(pub scale::Error);

impl fmt::Display for ArkScaleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        // use fmt::Display;
        self.0.fmt(f)
    }
}

impl ark_std::error::Error for ArkScaleError {}  // No source to return

pub fn scale_error_to_ark_error(error: scale::Error) -> io::Error {
    io::Error::new(io::ErrorKind::UnexpectedEof, ArkScaleError(error))
}

pub fn ark_error_to_scale_error(error: SerializationError) -> scale::Error {
    use SerializationError::*;
    // println!("{:?}",&error);
    match error {
        NotEnoughSpace => "Arkworks deserialization failed: NotEnoughSpace".into(),
        InvalidData => "Arkworks deserialization failed: InvalidData".into(),
        UnexpectedFlags => "Arkworks deserialization failed: UnexpectedFlags".into(),
        IoError(io_error) => {
            let err_msg: scale::Error = "Arkworks deserialization io error".into();
            let err_msg = err_msg.chain(format!("{}",&io_error));
            // ark_std::Error lacks downcasting https://github.com/arkworks-rs/std/issues/44
            #[cfg(feature = "std")]
            if let Some(boxed_dyn_error) = io_error.into_inner() {
                if let Ok(error) = boxed_dyn_error.downcast::<ArkScaleError>() {
                    return error.0;
                }
            }
            err_msg
        },
    }
}


/// Scale `Input` wrapped as Arkworks' `Read`
pub struct InputAsRead<'a,I: Input>(pub &'a mut I);

impl<'a,I: Input> Read for InputAsRead<'a,I> {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        panic!("At present Scale uses only read_exact, but if this changes then we should handle lengths correctly.");
        // assert_eq!(self.0.remaining_len(), Ok(Some(buf.len())));
        // println!("{:?}",self.0.remaining_len());
        // Avoid reading too much if the limit exists?!?
        /*
        let l = self.0.remaining_len()
        .map_err(scale_error_to_ark_error) ?
        .unwrap_or(buf.len());
        let l = core::cmp::min(l,buf.len());
        self.0.read(&mut buf[0..l]).map_err(scale_error_to_ark_error) ?;
        Ok(l)
        */
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        // scale's Input::read acts like Read::read_exact
        self.0.read(buf).map_err(scale_error_to_ark_error) ?;
        Ok(())
    }
}


/// Scale `Output` wrapped as Arkworks' `Write`
pub struct OutputAsWrite<'a,O: Output+?Sized>(pub &'a mut O);

impl<'a,I: Output+?Sized> Write for OutputAsWrite<'a,I> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>{
        // Scale `Output`s always succeed
        self.0.write(buf);
        // Scale `Output`s always succeed fully
        Ok(buf.len())
    }

    fn flush(&mut self) -> ArkResult<()> {
        // Scale `Output`s always succeed immediately
        Ok(())
    }
}

