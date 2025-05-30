use polars::prelude::*;
use std::path::Path;
use serde::de::StdError;
use crate::indicators::utils::utils::rolling;




pub(crate) fn ma(data : &mut DataFrame, n : i64) {
    let name = format!("MA_{}",n);
    let close = &data["close"];
    let ma_n = Series::new(&*name, rolling(close.clone(), n ));
    data.with_column(ma_n);

}

pub fn ema(data : &mut DataFrame, n : i64) {
    let name = format!("EMA_{}",n);
    let mut ema : Vec<f64> = vec![];
    let a = 2.0/(n as f64 + 1.0);
    let close = &data["close"];
    let ma_n = {
        let mut sum_n : f64 = 0.0;
        for i in 0..n {
            sum_n += close.f64().unwrap().get(i as usize).unwrap();
        }
        sum_n/n as f64
    };
    for i in 0..data.height(){
        if i < n as usize {
            ema.push(0.0);
        } else if i == n as usize {
            ema.push(a*ma_n + (1.0 - a)*close.f64().unwrap().get(i).unwrap());
        } else {
            ema.push(ema[i-1]* a + close.f64().unwrap().get(i).unwrap()*(1.0-a));
        }
    }
    
    data.with_column(Series::new(&name,ema));
    
}
