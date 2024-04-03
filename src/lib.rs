#![no_std]

pub mod contract;
mod dependencies;
mod errors;
mod storage;

#[cfg(test)]
extern crate std;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod testutils;
