#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

extern crate exitfailure;
extern crate playground_warp;
extern crate structopt;
extern crate warp;

use exitfailure::ExitFailure;
use playground_warp::cli::Cli;
use structopt::StructOpt;
use warp::Filter;

fn main() -> Result<(), ExitFailure> {
  let _args = Cli::from_args();

  let hi = warp::path("hello")
    .and(warp::header("user-agent"))
    .and(warp::path::param())
    .map(|param: String, agent: String| {
      format!("Hello {}, whose agent is {}", param, agent)
    });

  let server = warp::serve(warp::get(hi));
  println!("listening on port 8080");
  server.run(([127, 0, 0, 1], 8080));
  Ok(())
}
