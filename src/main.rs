extern crate clap;

use clap::Parser;
use grep_util::{Grep, GrepImpl};

fn main() {
    let grep = Grep::parse();
    let grep_impl = GrepImpl::new(grep);
    grep_impl.search();
}
