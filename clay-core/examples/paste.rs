use paste;


struct A(i32);
struct B(f32);

trait C {
    fn echo(&self);
}
impl C for A { fn echo(&self) { println!("{}", self.0); } }
impl C for B { fn echo(&self) { println!("{}", self.0); } }

macro_rules! de {
    ( $E:ident { $( $O:ident($C:ty) ),+ $(,)? } ) => {
        paste::item!{
            de!($E { $( $O([<T $O>] = $C) ),+ });
        }
    };
    ( $E:ident { $( $O:ident($T:ident = $C:ty) ),+ $(,)? } ) => {
        enum $E<
            $( $T = $C ),+
        > {
            $( $O($T), )+
        }

        impl<
            $( $T: C ),+
        > C for E<
            $( $T ),+
        > {
            fn echo(&self) {
                match self {
                    $( $E::$O(x) => x.echo(), )+
                }
            }
        }
    };
}

de!(E {
    A(A),
    B(B),
});

fn main() {
    let es = [E::A(A(1)), E::B(B(1.2))];
    for e in es.iter() {
        e.echo();
    }
}
