// Bitcoin transaction processing & database indexing daemon
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.


use std::io;

#[derive(Debug, Display)]
#[display_from(Debug)]
pub enum Error {
    IoError(io::Error),
    ZmqError(zmq::Error)
}

impl From<zmq::Error> for Error {
    fn from(err: zmq::Error) -> Self {
        Error::ZmqError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}