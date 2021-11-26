macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    };
    (print $e:expr)=>{
        println!("{}", stringify!{$e});
    };
}

fn main() {
    calculate! {
        eval 1 + 2
    }
    calculate! {
        print 1 + 2
    }
}
