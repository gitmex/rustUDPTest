use std::str;
use std::thread;
use std::net::UdpSocket;

struct panel{
	width: i32,
	height: i32,
	pixels: Vec<bool>,
}

struct flipdot{
	rows: i32,
	cols: i32,
	panels: Vec<panel>,
}

fn open_socket(portnum:i32){
	let socket = match UdpSocket::bind("0.0.0.0:5514") {
        Ok(s) => s,
 	   Err(e) => panic!("Nix Socket: {}", e)
    };

    let mut buf = [0; 2048];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                thread::spawn(move || {
                    println!("Wie gross bin ich?: {}", amt);
                    println!("Was steht in mir drin?: {}", src);
                    println!("{}", str::from_utf8(&buf).unwrap_or(""));
                });
            },
            Err(e) => {
                println!("Okay, das ging schief: {}", e);
            }
        }
    }	
}

fn initiate_panels(width: i32, height:i32, rows:i32, cols:i32){
	let panelcount=rows*cols;

	let mut flipper: flipdot;
	
	



	for i in 0..panelcount{
		let mut tmpPanel: panel; 
		tmpPanel = panel{width:width, height:height, pixels:vec![]};
		
		for j in 0..width*height{
			tmpPanel.pixels.push(false);			
			println!("{} - {}",i,j);		
		}		
	
		flipper.panels.push(tmpPanel);	
	}

	
}

fn print_screen(){
		
}

fn main() {
	let mut flipper: flipdot;
	flipper = flipdot {rows:0, cols:0, panels:vec![]};


	initiate_panels(16,20,9,6);
	println!("{}", flipper.panels.len());    
}

