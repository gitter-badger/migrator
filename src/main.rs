#![feature(convert, custom_derive)]
#[macro_use(bson, doc)]

extern crate bson;
extern crate mongodb as mongo;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::cmp::{PartialOrd, PartialEq, Ordering};
use std::fmt::Display;
use std::collections::BinaryHeap;

use mongo::ThreadedClient;

fn main() {

}
