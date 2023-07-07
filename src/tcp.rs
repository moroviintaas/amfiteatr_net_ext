use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::marker::PhantomData;
use speedy::{LittleEndian, Readable, Writable};
use sztorm::error::CommError;
use thiserror::Error;
use sztorm::comm::CommEndpoint;
use sztorm::protocol::DomainParameters;

//const BRIDGE_COMM_BUFFER_SIZE: usize = 256;
#[derive(Debug, Copy, Clone, Error)]
pub enum TcpCommError{
    #[error("Serialize Error")]
    SerializeError,
    #[error("Deserialize Error")]
    DeserializeError,
    #[error("Send Error")]
    SendError,
    #[error("Recv Error")]
    RecvError,
    #[error("TryRecv Error (empty)")]
    TryRecvEmptyError,
    #[error("TryRecv Error (disconnected)")]
    TryRecvDisconnectedError,
}
/*
impl From<TcpCommError> for CommError{
    fn from(value: TcpCommError) -> Self {
        match value{
            TcpCommError::SerializeError => Self::SerializeError,
            TcpCommError::DeserializeError => Self::DeserializeError,
            TcpCommError::SendError => Self::SendError,
            TcpCommError::RecvError => Self::RecvError,
            TcpCommError::TryRecvEmptyError => Self::TryRecvEmptyError,
            TcpCommError::TryRecvDisconnectedError => Self::TryRecvDisconnectedError
        }
    }
}
*/
pub struct TcpComm<OT, IT, E: Error, const  SIZE: usize>{
    stream: std::net::TcpStream,
    _ot: PhantomData<OT>,
    _it: PhantomData<IT>,
    _e: PhantomData<E>,
    _buff: PhantomData<[u8; SIZE]>
}

impl<OT, IT, E: Error, const  SIZE: usize> TcpComm<OT, IT, E, SIZE>{
    pub fn new(stream : std::net::TcpStream) -> Self{
        Self{stream, _ot: PhantomData::default(), _it: PhantomData::default(), _e: PhantomData::default(), _buff: PhantomData::default()}
    }
}

impl<'a, OT, IT, E: Error, const  SIZE: usize> CommEndpoint for TcpComm<OT, IT, E, SIZE>
where OT: Writable<LittleEndian> + Debug, IT: Readable<'a, LittleEndian> + Debug,
E: From<TcpCommError>{
    type OutwardType = OT;

    type InwardType = IT;

    type Error = E;


    fn send(&mut self, message: OT) -> Result<(), E> {

        let mut buffer = [0u8; SIZE];
        if message.write_to_buffer(&mut buffer).is_err(){
            return Err(TcpCommError::SerializeError.into())
        }
        match self.stream.write_all(&buffer){
            Ok(_) => Ok(()),
            Err(_) => Err(TcpCommError::SendError.into()),
        }
    }
    fn recv(&mut self) -> Result<IT, E> {
        self.stream.set_nonblocking(false).unwrap();
        let mut buffer = [0u8; SIZE];
        let mut received = false;
        while !received {
            match self.stream.read(&mut buffer){
                Ok(0) => {},
                Ok(_) => {
                    received = true;
                },
                Err(_e) => {return Err(TcpCommError::RecvError.into())}
            }
        }
        match IT::read_from_buffer_copying_data(&buffer){
            Ok(m) => Ok(m),
            Err(_) => Err(TcpCommError::DeserializeError.into())
        }
    }
    fn try_recv(&mut self) -> Result<IT, E> {
        let mut buffer = [0u8; SIZE];
        self.stream.set_nonblocking(true).unwrap();

        match self.stream.read(&mut buffer){
            Ok(0) => {
                //debug!("TryRecvError");
                Err(TcpCommError::TryRecvEmptyError.into())
            }
            Ok(_n) => {
                //debug!("Tcp TryRecv with {} bytes", n);
                match IT::read_from_buffer_copying_data(&buffer){
                    Ok(m) => Ok(m),
                    Err(_) => Err(TcpCommError::DeserializeError.into())
                }
            },
            Err(_e) => {

                Err(TcpCommError::TryRecvDisconnectedError.into())
            }
        }


    }
}

pub type TcpCommK1<OT, IT, E> = TcpComm<OT, IT, E, 1024>;
pub type TcpCommK2<OT, IT, E> = TcpComm<OT, IT, E, 2048>;
pub type TcpComm512<OT, IT, E> = TcpComm<OT, IT, E, 512>;

impl<Spec: DomainParameters> From<TcpCommError> for CommError<Spec>{
    fn from(value: TcpCommError) -> Self {
        match value{
            TcpCommError::SerializeError => CommError::SerializeError,
            TcpCommError::DeserializeError => CommError::DeserializeError,
            TcpCommError::SendError => CommError::SendErrorUnspecified,
            TcpCommError::RecvError => CommError::RecvErrorUnspecified,
            TcpCommError::TryRecvEmptyError => CommError::TryRecvErrorEmptyUnspecified,
            TcpCommError::TryRecvDisconnectedError => CommError::TryRecvErrorDisconnectedUnspecified
        }
    }
}