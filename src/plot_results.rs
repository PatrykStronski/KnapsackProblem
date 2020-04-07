use plotters::prelude::*;

fn get_min(winners: Vec<u32>, max_winner: u32) -> i32 {
    let mut min: u32 = max_winner;
    for winner in winners {
        if winner < min {
            min = winner;
        }
    }
    return min as i32;
}

fn get_max(winners: Vec<u32>) -> i32 {
    let mut max: u32 = 0;
    for winner in winners {
        if winner >  max {
            max = winner;
        }
    }
    return (max + 500) as i32;
}

pub fn plot_results(winners: Vec<u32>, changing_value: Vec<f32>, label: String, caption: String) {
    let filename = format!("{}_diagram.png", label);
    let max_winner = get_max(winners.to_vec());
    let min_winner = get_min(winners.to_vec(), max_winner as u32);
    println!("min_winner {}",min_winner);
    println!("max_winner {}",max_winner);
    let root = BitMapBackend::new(&filename, (640, 840)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut ctx = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(&caption, ("Arial", 15))
        .build_ranged(changing_value[0]..changing_value[changing_value.len()-1], min_winner..max_winner)
        .unwrap();
    ctx.configure_mesh().draw().unwrap();
    ctx.draw_series(
        LineSeries::new(
            (0..).zip(changing_value.iter()).map(|(idx, val)| {
                (*val, (winners[idx] as i32)/5)
            }),
            &BLUE,
        )
    ).unwrap();
}
