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


//static mut flipper: flipdot = flipdot{rows:0, cols:0, panels:vec![]}; 

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

fn initiate_panels(width: i32, height:i32, rows:i32, cols:i32) -> flipdot{
	let panelcount=rows*cols;

	let mut flipper: flipdot;

	flipper = flipdot{rows:rows, cols:cols, panels:vec![]};
	

	for i in 0..panelcount{
		let mut tmpPanel: panel; 
		tmpPanel = panel{width:width, height:height, pixels:vec![]};
		
		for j in 0..width*height{
			tmpPanel.pixels.push(false);			
		}		
	
		flipper.panels.push(tmpPanel);	
	}

	println!("{}", flipper.panels.len());
	println!("{}", flipper.panels[0].pixels.len() );
	return flipper;
}

fn print_screen(flipper:flipdot){
	println!("{}", flipper.panels[0].pixels.len() );
}

fn translate_px_to_panel(flipper:flipdot,x:i32,y:i32) -> (i32,i32){
	return (0,0);
}

fn main() {
	
	let mut flipper: flipdot;	

	flipper = initiate_panels(16,20,9,6);
	print_screen(flipper);	
	    
}

