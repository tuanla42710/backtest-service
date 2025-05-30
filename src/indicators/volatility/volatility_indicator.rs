use polars::prelude::*;
use crate::indicators::utils::utils::*;

fn bollinger_band(data : &mut DataFrame,n : i64, k : i32){
    let close = data.column("close").unwrap().f64().unwrap();
    
    let std = Series::new("std",rolling_std(close.clone().into_series(), n));
    
    let middle_band = Series::new("middle_band",rolling_std(close.clone().into_series(), n));
    
    let mut upper_band = middle_band.clone() + &std*k as f64;
    let mut lower_band = middle_band.clone() - &std*k as f64;
    
    data.with_column(middle_band);
    data.with_column(upper_band.rename("upper_band").clone());
    data.with_column(lower_band.rename("lower_band").clone());
    
    
}