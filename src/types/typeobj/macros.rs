#[macro_export]
macro_rules! unary {
    ($h:ident, signed default $ty1:ident $ty2:ident) => {
        unary!($h, $ty1, "_un_add", |x: &Vec<Value>| Some(x[0].to_owned()));
        unary!($h, $ty1, "_un_sub", |x: &Vec<Value>| Some(Value::$ty2(
            get_param!(x, 0, $ty2).checked_neg()?
        )));
        unary!($h, $ty1, "_not", |x: &Vec<Value>| Some(Value::Bool(
            get_param!(x, 0, $ty2) == 0
        )));
    };
    ($h:ident, unsigned default $ty1:ident $ty2:ident) => {
        unary!($h, $ty1, "_un_add", |x: &Vec<Value>| Some(x[0].to_owned()));
        unary!($h, $ty1, "_not", |x: &Vec<Value>| Some(Value::Bool(
            get_param!(x, 0, $ty2) == 0
        )));
    };
    ($h:ident, float $ty1:ident $ty2:ident) => {
        unary!($h, $ty1, "_un_add", |x: &Vec<Value>| Some(x[0].to_owned()));
        unary!($h, $ty1, "_un_sub", |x: &Vec<Value>| Some(Value::$ty2(
            get_param!(x, 0, $ty2).neg()
        )));
        unary!($h, $ty1, "_not", |x: &Vec<Value>| Some(Value::Bool(
            get_param!(x, 0, $ty2) == 0.0 || get_param!(x, 0, $ty2) == -0.0
        )));
    };
    ($h:ident, $ty:ident, $n:literal, $f:expr) => {
        $h.insert(
            $n,
            Proc::Builtin {
                f: $f,
                signature: vec![(vec![$ty.to_owned()], $ty.to_owned())],
            },
        );
    };
}

#[macro_export]
macro_rules! binary {
    ($h:ident, $ty:ident, $n:literal, [$($o:expr),+], $f:expr) => {
        $h.insert(
            $n,
            Proc::Builtin {
                f: $f,
                signature: [$($o),+].into_iter().map(|o| (
                    vec![$ty.to_owned(), o.to_owned()],
                    $ty.to_owned(),
                )).collect(),
            },
        );
    };
    ($h:ident, $ty:ident, $n:literal, [$($o:expr),+], $r:expr, $f:expr) => {
        $h.insert(
            $n,
            Proc::Builtin {
                f: $f,
                signature: [$($o),+].into_iter().map(|o| (
                    vec![$ty.to_owned(), o.to_owned()],
                    $r.to_owned(),
                )).collect(),
            },
        );
    };
}

#[macro_export]
macro_rules! get_param {
    ($x:ident, $i:literal, $v:ident) => {
        if let Value::$v(v) = $x[$i] {
            v
        } else {
            unreachable!()
        }
    };
}

#[macro_export]
macro_rules! typecast_to_type {
    ($v:ident) => {
        Value::Type($v.to_owned())
    };
}

#[macro_export]
macro_rules! typecast_int {
    ($v:ident => str, $x:ident) => {
        Value::Str(get_param!($x, 0, $v).to_string())
    };
    ($v:ident => bool, $x:ident) => {
        Value::Bool(get_param!($x, 0, $v) == 0)
    };
    ($v:ident => into bool, $x:ident) => {
        Value::Bool(get_param!($x, 0, $v) == 0.into())
    };
    ($v:ident => f64, $x:ident) => {
        Value::F64(get_param!($x, 0, $v) as f64)
    };
    ($v:ident => f32, $x:ident) => {
        Value::F32(get_param!($x, 0, $v) as f32)
    };
    ($v:ident => f16, $x:ident) => {
        Value::F16(f16::from_f64(get_param!($x, 0, $v) as f64))
    };
    (big $v:ident => f64, $x:ident) => {
        Value::F64(get_param!($x, 0, $v).to_f64()?)
    };
    (big $v:ident => f32, $x:ident) => {
        Value::F32(get_param!($x, 0, $v).to_f32()?)
    };
    (big $v:ident => f16, $x:ident) => {
        Value::F16(f16::from_f64(get_param!($x, 0, $v).to_f64()?))
    };
    ($v:ident => $vo:ident, $x:ident) => {
        Value::$vo(get_param!($x, 0, $v).try_into().ok()?)
    };
}

#[macro_export]
macro_rules! typecast_float {
    ($v:ident => str, $x:ident) => {
        Value::Str(get_param!($x, 0, $v).to_string())
    };
    ($v:ident => bool, $x:ident) => {
        Value::Bool(get_param!($x, 0, $v) == 0.0.into())
    };
    (F32 => f16, $x:ident) => {
        Value::F16(f16::from_f32(get_param!($x, 0, F32)))
    };
    (F64 => f16, $x:ident) => {
        Value::F16(f16::from_f64(get_param!($x, 0, F64)))
    };
    ($v:ident => $vo:ident $f:ident, $x:ident) => {
        Value::$vo(get_param!($x, 0, $v).$f()?)
    };
}

#[macro_export]
macro_rules! arith_opr_num {
    ($h:ident, default $ty1:ident $ty2:ident) => {{
        arith_opr_num!($h, $ty1 $ty2, "_add" checked_add);
        arith_opr_num!($h, $ty1 $ty2, "_sub" checked_sub);
        arith_opr_num!($h, $ty1 $ty2, "_mul" checked_mul);
        arith_opr_num!($h, $ty1 $ty2, "_div" checked_div);
        arith_opr_num!($h, $ty1 $ty2, "_rem" checked_rem);
    }};
    ($h:ident, big default $ty1:ident $ty2:ident) => {{
        arith_opr_num!($h, $ty1 $ty2, "_add" big checked_add);
        arith_opr_num!($h, $ty1 $ty2, "_sub" big checked_sub);
        arith_opr_num!($h, $ty1 $ty2, "_mul" big checked_mul);
        arith_opr_num!($h, $ty1 $ty2, "_div" big checked_div);
        binary!($h, $ty1, "_rem", [$ty1], |x: &Vec<Value>| Some(
            Value::$ty2(get_param!(x, 0, $ty2).rem(&get_param!(x, 1, U32)))
        ));
    }};
    ($h:ident, float default $ty1:ident $ty2:ident) => {{
        arith_opr_num!($h, $ty1 $ty2, "_add" float add);
        arith_opr_num!($h, $ty1 $ty2, "_sub" float sub);
        arith_opr_num!($h, $ty1 $ty2, "_mul" float mul);
        arith_opr_num!($h, $ty1 $ty2, "_div" float div);
        arith_opr_num!($h, $ty1 $ty2, "_rem" float rem);
    }};
    ($h:ident, $ty1:ident $ty2:ident, $fn_name:literal $rust_fn:ident) => {
        binary!($h, $ty1, $fn_name, [$ty1], |x: &Vec<Value>| Some(
            Value::$ty2(get_param!(x, 0, $ty2).$rust_fn(get_param!(x, 1, $ty2))?)
        ));
    };
    ($h:ident, $ty1:ident $ty2:ident, $fn_name:literal big $rust_fn:ident) => {
        binary!($h, $ty1, $fn_name, [$ty1], |x: &Vec<Value>| Some(
            Value::$ty2(get_param!(x, 0, $ty2).$rust_fn(&get_param!(x, 1, $ty2))?)
        ));
    };
    ($h:ident, $ty1:ident $ty2:ident, $fn_name:literal float $rust_fn:ident) => {
        binary!($h, $ty1, $fn_name, [$ty1], |x: &Vec<Value>| Some(
            Value::$ty2(get_param!(x, 0, $ty2).$rust_fn(&get_param!(x, 1, $ty2)))
        ));
    }
}
#[macro_export]
macro_rules! comp_opr_num {
    ($h:ident, default $ty1:ident $ty2:ident) => {{
        comp_opr_num!($h, $ty1 $ty2, "_eq" eq);
        comp_opr_num!($h, $ty1 $ty2, "_ne" ne);
        comp_opr_num!($h, $ty1 $ty2, "_gt" gt);
        comp_opr_num!($h, $ty1 $ty2, "_ge" ge);
        comp_opr_num!($h, $ty1 $ty2, "_lt" lt);
        comp_opr_num!($h, $ty1 $ty2, "_le" le);
    }};
    ($h:ident, $ty1:ident $ty2:ident, $fn_name:literal $rust_fn:ident) => {
        binary!($h, $ty1, $fn_name, [$ty1], BOOL_T, |x: &Vec<Value>| Some(
            Value::Bool(get_param!(x, 0, $ty2).$rust_fn(&get_param!(x, 1, $ty2)))
        ));
    };
}
#[macro_export]
macro_rules! concat_vals {
    ($h:ident, $ty1:ident) => {
        binary!($h, $ty1, "_concat", [Type::Any], STR_T, |x: &Vec<Value>| {
            Some(Value::Str(format!("{}{}", x[0], x[1])))
        });
    };
}
