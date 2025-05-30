use polars::prelude::*;

pub fn rolling(d : Series, n : i64) -> Vec<f64>{

    let mut rc : Vec<f64> = vec![];
    let d : Vec<f64> = d.f64().unwrap().into_no_null_iter().collect();
    let mut sum: f64 = 0.0;
    for i in 0..d.len(){
        sum += d[i];
        if i >= n  as usize {
            sum -= d.get(i - n  as usize).unwrap();
            rc.push(sum/n as f64);
        }
        else if i == (n-1) as usize {
            rc.push(sum/n as f64);
        }
        else {
            rc.push(0.0);
        }

    }
    return rc;
}

pub fn rolling_std(d : Series, n : i64) -> Vec<f64> {

    let avg = Series::new("sum",rolling(d.clone(), n )).f64().unwrap().clone();
    let square = d.clone().f64().unwrap().apply(|x| x*x);
    let square = Series::new("sum",rolling(square.into_series(),n)).f64().unwrap().clone();
    let std =  avg.into_iter().zip(square.into_iter())
        .map(|(a,b)| match (a , b){
            (Some(val1), Some(val2)) => (val2*(n as f64/(n as f64-1.0)) - val1*val1*(n as f64/(n as f64-1.0))).sqrt(),
            _ => 0.0
        }).collect::<Vec<f64>>();
    std

}