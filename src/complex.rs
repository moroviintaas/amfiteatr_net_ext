

use speedy::{Writable, Readable, LittleEndian};
use sztorm::{CommEndpoint, SyncComm, error::CommError};
use std::{fmt::Debug, sync::mpsc::{RecvError, SendError, TryRecvError}};

use crate::tcp::{TcpComm, TcpCommError};

pub enum ComplexComm<OT, IT, E: std::error::Error, const  SIZE: usize>{
    StdSync(SyncComm<OT, IT, E>),
    Tcp(TcpComm<OT, IT, E, SIZE>)

}

impl <'a, OT, IT, E: std::error::Error, const  SIZE: usize> CommEndpoint for ComplexComm<OT, IT, E, SIZE>
where OT: Writable<LittleEndian> + Debug, IT: Readable<'a, LittleEndian> + Debug,
E: std::error::Error + From<TcpCommError> + From<RecvError> + From<SendError<OT>> + From<TryRecvError> + From<SendError<IT>>{
    type OutwardType = OT;

    type InwardType = IT;

    type Error = E;

    fn send(&mut self, message: Self::OutwardType) -> Result<(), Self::Error> {
        match self{
            ComplexComm::StdSync(comm) => comm.send(message),
            ComplexComm::Tcp(comm) => comm.send(message),
        }
    }

    fn recv(&mut self) -> Result<Self::InwardType, Self::Error> {
        match self{
            ComplexComm::StdSync(comm) => comm.recv(),
            ComplexComm::Tcp(comm) => comm.recv(),
        }
    }

    fn try_recv(&mut self) -> Result<Self::InwardType, Self::Error> {
        match self{
            ComplexComm::StdSync(comm) => comm.try_recv(),
            ComplexComm::Tcp(comm) => comm.try_recv(),
        }
    }
}

pub type ComplexComm2048<OT, IT, E> = ComplexComm<OT, IT, E, 2048>;