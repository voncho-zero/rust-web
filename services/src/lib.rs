pub mod base;
#[macro_use]
extern crate rbatis;
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub async fn init_db(addr: &String) -> RBatis{
    // fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    let rb = RBatis::new();
    // mysql
    rb.link(MysqlDriver{}, addr).await.unwrap();
    return rb;
}
