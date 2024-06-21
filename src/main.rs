
enum Ticket{
    Backstage(f64,String),
    Vip(f64,String),
    Standard(f64),
}


// option in rust

struct Survay{
    q1:Option<i32>,
    q2:Option<bool>,

    q3:Option<String>,




}


// activiry optional data in rust

struct Locker{

    name : String,
    number: Option<i32>,


}


fn main() {
   

    let my_tickets=vec![

        Ticket::Backstage(20.0,"Arman".to_owned()),
        Ticket::Vip(40.0,"ali".to_owned()),
        Ticket::Standard(10.0),
    ];
    for tick in my_tickets{
        match tick {
            Ticket::Backstage(20.0,name )=> println!("{:?}",name),
            _=>(),
            
        }
    }

    // option in rust

    let responce = Survay{
        q1:Some(12),
        q2:Some(true),
        q3:Some("A".to_owned()),
    };
    match responce.q1 {
        Some(ans)=> println!("q1 : {:?}",ans),
        None => println!(" no responce")
        
    }
    match responce.q2 {
        Some(ans)=> println!("q2 : {:?}",ans),
        None => println!(" no responce")
        
    }
    match responce.q3 {
        Some(ans)=> println!("q3 : {:?}",ans),
        None => println!(" no responce")
        
    }

    let student = Locker{
        name : "arman".to_owned(),
        number : Some(4),
    }; 
    println!("Student name : {:?}",student.name);
    match student.number {
        Some(num)=> println!("{:?}",num),
        None => println!(" no number assigned"),
        
    }

    // activity optional data in rust
    
}
