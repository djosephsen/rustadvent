pub fn compute(mass: f32) -> f32{
    (mass/3.0).floor() - 2.0
}

fn main() {
    let input = [83326.0,84939.0,135378.0,105431.0,119144.0,124375.0,138528.0,88896.0,98948.0,85072.0,112576.0,144497.0,112824.0,98892.0,81551.0,139462.0,73213.0,93261.0,130376.0,118425.0,132905.0,54627.0,134676.0,140435.0,131410.0,128441.0,96755.0,94866.0,89490.0,122118.0,106596.0,77531.0,84941.0,57494.0,97518.0,136224.0,69247.0,147209.0,92814.0,63436.0,79819.0,109335.0,85698.0,110103.0,79072.0,52282.0,73957.0,68668.0,105394.0,149663.0,91954.0,66479.0,55778.0,126377.0,75471.0,75662.0,71910.0,113031.0,133917.0,76043.0,65086.0,117882.0,134854.0,60690.0,67495.0,62434.0,67758.0,95329.0,123078.0,128541.0,108213.0,93543.0,147937.0,148262.0,56212.0,148586.0,73733.0,110763.0,149243.0,133232.0,95817.0,68261.0,123872.0,93764.0,147297.0,51555.0,110576.0,89485.0,109570.0,88052.0,132786.0,70585.0,105973.0,85898.0,149990.0,114463.0,147536.0,67786.0,139193.0,112322.0];

    // part 1 (
    let mut total = 0.0;
    for component in input.iter() {
        total += compute(*component)
    }
    println!("part 1: {}",total); 

    // part 2 (mass of the fuel of the fuel of the fuel...)
    total = 0.0;
    for component in input.iter() {
        let component_fuel = compute(*component);
        total += component_fuel;
        let mut additional_fuel = compute(component_fuel);
        total += additional_fuel;
        loop {
            additional_fuel = compute(additional_fuel);
            if additional_fuel <= 0.0 {break}
            total += additional_fuel;
        }
    }
    println!("part 2: {}",total); 
}
