use rand::Rng;
use csv::Writer;
use std::error::Error;

struct Obj {
    weight: u32,
    price: u32,
    size: u32
}

fn save_to_file(arr: &mut[Obj],n:u32,w:u32,s:u32, out_file: String) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(out_file)?;
    wtr.write_record(&[n.to_string(),w.to_string(),s.to_string()])?;
    for obj in arr{
        wtr.write_record(&[obj.weight.to_string(), obj.size.to_string(),obj.price.to_string()])?;
    }
    wtr.flush()?;
    Ok(())
}

fn lower_down(current: u32, surplus: f32) -> u32 {
    let current_f = current as f32;
    let lowering = current_f * surplus;
    if lowering < 0.5 && current > 1 {
        return current - 1;
    } else {
        if current > 1 {
            return current - (lowering.round() as u32);
        }
    }
    return current;
}

fn make_smooth(arr: &mut[Obj], surplus: f32, field: String) {
    for obj in arr {
        match field.as_str() {
            "size" => obj.size=lower_down(obj.size, surplus),
            "weight" => obj.weight=lower_down(obj.weight, surplus),
            _ => println!("wrong property"),
        };
    }
}

fn evaluate_surplus(arr: &mut[Obj], control_p: u32, property: String) -> f32 {
    let mut sum = 0;
    for obj in arr {
        match property.as_str() {
            "size" => sum+=obj.size,
            "weight" => sum+=obj.weight,
            _ => println!("wrong property"),
        }
    }
    let sum_f = sum as f32;
    let numerator = sum_f - 2.0 * control_p as f32;
    let denominator = sum_f;
    return numerator / denominator;
}

fn smooth_set(arr: &mut[Obj], w: u32, s:u32) {
    let mut s_ok: bool = false;
    let mut w_ok: bool = false;
    while !s_ok || !w_ok {
        let s_surplus = evaluate_surplus(arr, s, "size".to_string());
        let w_surplus = evaluate_surplus(arr, w, "weight".to_string());
        println!("s: {} \t w: {}",s_surplus,w_surplus);
        if s_surplus < 0.0 { s_ok = true; }
        if w_surplus < 0.0 { w_ok = true; }
        if !w_ok { make_smooth(arr, w_surplus, "weight".to_string()); }
        if !s_ok { make_smooth(arr, s_surplus, "size".to_string()); }
    }
}

fn generate_one(n: u32, w: u32, s:u32) -> Obj {
    let unit = Obj {
        weight: rand::thread_rng().gen_range(1,(10*w/n) as u32),
        price: rand::thread_rng().gen_range(1,n),
        size: rand::thread_rng().gen_range(1,(10*s/n) as u32)
    };
    return unit;
}

/*
 * @param n - number of objects to choose (int)
 * @param w - maximum weight
 * @param s - maximum size
 * @param output_file - filename*/

pub fn generate(n: usize, w: u32, s: u32, output_file: String) {
    let mut arr = Vec::with_capacity(n);
    for _x in 0..n {
        arr.push(generate_one(n as u32, w, s));
    }
    smooth_set(&mut arr, w, s);
    let res = save_to_file(&mut arr,n as u32, w, s, output_file);
    match res {
        Ok(v) => println!("Written correctly, version {:?}",v),
        Err(v) => println!("ERROR: {:?}", v),
    };
}
