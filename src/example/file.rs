use std::fs::File;
pub fn file() {
  crate::dev_log("file");
  let res = File::open("/Cargo.toml");

  // match &res {
  //   Ok(file) => {
  //     println!("read file ok");
  //   },
  //   Err(error) => {
  //     println!("read file error {}", error);
  //   }
  // }

  // println!("{:?}", res);

  // make some error info to developer
  // res.unwrap();

  // res.expect("Failed to open hello.txt");

  use std::net::IpAddr;

  let home: IpAddr = "127.0.0.1".parse().unwrap();

  crate::dev_log("file done")
}
