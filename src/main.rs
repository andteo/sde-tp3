use std::env;

fn adunare(a:f32, b:f32)->f32{
    return a+b;
}

fn scadere(a:f32, b:f32)->f32{
    return a-b;
}

fn inmultire(a:f32, b:f32)->f32{
    return a*b;
}

fn impartire(a:f32, b:f32)->Option<f32>{
    if b==0.0{
        return None;
    } else{
        return Some(a/b);
    }
}

fn imprest(a:i32, b:i32)->Option<i32>{
    if b==0{
        return None;
    } else{
        return Some(a&b);
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();

    //let deimpartit = 10f32; ex1
    //let impartitor = 0f32;

    /*let deimpartit= args[1].parse().unwrap();  //unwrap sare verificarea in tipuri gen option
    let impartitor= args[2].parse().unwrap();
    match impartire(deimpartit, impartitor){
        Some(rez)=> println!("Rezultatul este: {}", rez),
        None=> println!("Nu se poate!")
    }*/ //ex2
    /*if args.len()>=3{
    let o=args[1].as_str();
    let a=args[2].parse().unwrap();
    let b=args[3].parse().unwrap();
    let rezultatimp:f32;
    let rest:i32;
    match impartire(a, b){
        Some(rez)=> rezultatimp=rez,
        None=> {println!("Nu se poate imp la 0");
            std::process::exit(-1)}
    }

    match imprest(a as i32, b as i32){
        Some(rez)=> rest=rez,
        None=> {println!("Nu se poate imp la 0");
            std::process::exit(-1)}
    }
    match o {
        "add"=>println!("Rezultatul adunarii este: {}", adunare(a, b)),
        "sub"=>println!("Rezultatul scaderii este: {}", scadere(a, b)),
        "mul"=>println!("Rezultatul scaderii este: {}", inmultire(a, b)),
        "div"=>println!("Rezultatul impartirii este: {}", rezultatimp),
        "rem"=>println!("Restul e: {}", rest),
        _=> std::process::exit(-1)
    }
    }else {
        println!("Problema");
        std::process::exit(-1);
    } ex3*/
    

    


    
}
