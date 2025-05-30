use polars::prelude::*;
use crate::indicators::utils::utils::rolling;
pub(crate) fn rsi(data: &mut DataFrame, n : i64) -> (){
    let close = &data["close"];
    let lag_price = close.shift(1);

    let gain = close - &lag_price;
    let loss = &lag_price - close;

    let adjust_gain = gain.f64().unwrap().apply(|v| if v > 0.0 {v} else {0.0});
    let adjust_loss = loss.f64().unwrap().apply(|v| if v > 0.0 {v} else {0.0});

    let avg_gain = Series::new("avg_gain",rolling(adjust_gain.clone().into_series(), n ));
    let avg_loss = Series::new("avg_loss",rolling(adjust_loss.clone().into_series(), n ));

    let lag_avg_gain = avg_gain.shift(1);
    let lag_avg_loss = avg_loss.shift(1);

    let new_gain = (&lag_avg_gain*13 + adjust_gain.into_series())/14;
    let new_loss = (&lag_avg_loss*13 + adjust_loss.into_series())/14;

    let ri = new_gain/new_loss;

    let rsi = ri.f64().unwrap().apply(|v| {100.0 - 100.0/( 1.0 + v )});
    data.with_column(rsi.into_series().rename("rsi").clone()).expect("fail");

}