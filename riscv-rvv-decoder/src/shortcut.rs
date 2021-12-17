// process macro like this:
// vadd.vv         31..26=0x00 vm vs2 vs1 14..12=0x0 vd 6..0=0x57
#[macro_export]
macro_rules! define {
    ($name:ident $($tts:tt)*) => {
        let mut $name = vec![];
        define!(@accum $name $($tts)*);
    };
    (@accum $name:ident  $bit:literal=$value:literal $($tts:tt)*) => {
        $name.push(Field {start : 0xFFFF, length : 1, value : Some($value)});
        define!(@accum $name $($tts)*);
    };
    (@accum $name:ident rd $($tts:tt)*) => {
        $name.push(Field {start : 0xFFFF, length : 5, value : None});
        define!(@accum $name  $($tts)*);
    };
    (@accum $name:ident vd $($tts:tt)*) => {
        $name.push(Field {start : 0xFFFF, length : 5, value : None});
        define!(@accum $name  $($tts)*);
    };
    (@accum $name:ident  rs1 $($tts:tt)*) => {
        $name.push(Field {start : 0xFFFF, length : 5, value : None});
        define!(@accum $name  $($tts)*);
    };
    (@accum $name:ident  rs2 $($tts:tt)*) => {
        $name.push(Field {start : 0xFFFF, length : 5, value : None});
        define!(@accum $name  $($tts)*);
    };
    (@accum $name:ident  nf $($tts:tt)*) => {
        $name.push(Field {start : 0xFFFF, length : 3, value : None});
        define!(@accum $name  $($tts)*);
    };
    (@accum $name:ident vs1 $($tts:tt)*) => {
        $name.push(Field {start : 0xFFFF, length : 5, value : None});
        define!(@accum $name  $($tts)*);
    };
    (@accum $name:ident  vs2 $($tts:tt)*) => {
        $name.push(Field {start : 0xFFFF, length : 5, value : None});
        define!(@accum $name  $($tts)*);
    };
    (@accum $name:ident  vs3 $($tts:tt)*) => {
        $name.push(Field {start : 0xFFFF, length : 5, value : None});
        define!(@accum $name  $($tts)*);
    };
    (@accum $name:ident vm $($tts:tt)*) => {
        $name.push(Field {start : 0xFFFF, length : 1, value : None});
        define!(@accum $name $($tts)*);
    };
    (@accum $name:ident  $hi:literal..$lo:literal=$value:literal $($tts:tt)*) => {
        $name.push(Field {start : $lo, length : $hi - $lo + 1, value : Some($value)});
        define!(@accum $name $($tts)*);
    };
    (@accum $name:ident) => {
    };
}
