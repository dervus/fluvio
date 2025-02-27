use fluvio_test_derive::fluvio_test;
#[warn(unused_imports)]
use fluvio_test_util::test_meta::TestCase;
use structopt::StructOpt;
use std::any::Any;
use fluvio_test_util::test_meta::TestOption;

#[derive(Debug, Clone, StructOpt, Default, PartialEq)]
#[structopt(name = "Fluvio Test Example")]
pub struct RunTestOption {}

impl TestOption for RunTestOption {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[fluvio_test(topic = "test")]
pub fn run(mut test_driver: TestDriver, test_case: TestCase) {
}

fn main() {

}