use std::{
    cmp::{Eq, PartialOrd},
    collections::HashMap,
    fmt::Display,
    ops::{Add, Div, Mul, Rem, Sub},
    sync::Arc,
};

use num_traits::{
    CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedSub, Float, Signed,
    Unsigned, Zero,
};

use crate::{
    primitives::{proc_t::generic_proc, ANY_T, BOOL_T, STR_T, TYPE_T},
    types::{
        r#type::Type,
        value::{BuiltinFunction, Proc, Value, ValueInner},
    },
};

pub fn get_param<T: TryFrom<Value>>(x: &[Value], i: usize) -> Option<T> {
    T::try_from(x[i].to_owned()).ok()
}

pub fn unary<'a>(
    h: &mut HashMap<&'a str, Value>,
    n: &'a str,
    f: Arc<BuiltinFunction>,
    arg_ty: Arc<Type>,
    ret_ty: Arc<Type>,
) {
    h.insert(
        n,
        Value::Proc(Proc::Builtin {
            f,
            ty: generic_proc(vec![arg_ty], ret_ty),
        }),
    );
}

pub fn unary_signed_default<T: Signed + CheckedNeg + Into<Value> + TryFrom<Value>>(
    h: &mut HashMap<&str, Value>,
    this_ty: Arc<Type>,
) {
    unary(
        h,
        "_un_add",
        Arc::new(|x: &Vec<Value>| Some(x[0].to_owned())),
        Arc::clone(&this_ty),
        Arc::clone(&this_ty),
    );
    unary(
        h,
        "_un_sub",
        Arc::new(|x: &Vec<Value>| Some({ get_param::<T>(&x, 0)?.checked_neg()?.into() })),
        Arc::clone(&this_ty),
        Arc::clone(&this_ty),
    );
    unary(
        h,
        "_not",
        Arc::new(|x: &Vec<Value>| Some(get_param::<T>(&x, 0)?.is_zero().into())),
        Arc::clone(&this_ty),
        Arc::clone(&BOOL_T),
    );
}

pub fn unary_unsigned_default<T: Unsigned + Into<Value> + TryFrom<Value>>(
    h: &mut HashMap<&str, Value>,
    this_ty: Arc<Type>,
) {
    unary(
        h,
        "_un_add",
        Arc::new(|x: &Vec<Value>| Some(x[0].to_owned())),
        Arc::clone(&this_ty),
        Arc::clone(&this_ty),
    );
    unary(
        h,
        "_not",
        Arc::new(|x: &Vec<Value>| Some(get_param::<T>(&x, 0)?.is_zero().into())),
        Arc::clone(&this_ty),
        Arc::clone(&BOOL_T),
    );
}

pub fn unary_float_default<T: Float + Into<Value> + TryFrom<Value>>(
    h: &mut HashMap<&str, Value>,
    this_ty: Arc<Type>,
) {
    unary(
        h,
        "_un_add",
        Arc::new(|x: &Vec<Value>| Some(x[0].to_owned())),
        Arc::clone(&this_ty),
        Arc::clone(&this_ty),
    );
    unary(
        h,
        "_un_sub",
        Arc::new(|x: &Vec<Value>| Some({ get_param::<T>(&x, 0)?.neg().into() })),
        Arc::clone(&this_ty),
        Arc::clone(&this_ty),
    );
    unary(
        h,
        "_not",
        Arc::new(|x: &Vec<Value>| {
            Some(
                (get_param::<T>(&x, 0)?.is_zero() || get_param::<T>(&x, 0)?.eq(&T::neg_zero()))
                    .into(),
            )
        }),
        Arc::clone(&this_ty),
        Arc::clone(&BOOL_T),
    );
}

pub fn binary<'a>(
    h: &mut HashMap<&'a str, Value>,
    n: &'a str,
    f: Arc<BuiltinFunction>,
    arg1_ty: Arc<Type>,
    arg2_ty: Arc<Type>,
    ret_ty: Arc<Type>,
) {
    h.insert(
        n,
        Value::Proc(Proc::Builtin {
            f,
            ty: generic_proc(vec![arg1_ty, arg2_ty], ret_ty),
        }),
    );
}

#[macro_export]
macro_rules! typecast_int {
    ($v:ty => str, $x:ident) => {
        Value::Str(get_param::<$v>($x, 0)?.to_string())
    };
    ($v:ty => bool, $x:ident) => {
        Value::Bool(get_param::<$v>($x, 0)? == 0)
    };
    ($v:ty => f64, $x:ident) => {
        Value::F64(get_param::<$v>($x, 0)? as f64)
    };
    ($v:ty => f32, $x:ident) => {
        Value::F32(get_param::<$v>($x, 0)? as f32)
    };
    ($v:ty => f16, $x:ident) => {
        Value::F16(f16::from_f64(get_param::<$v>($x, 0)? as f64))
    };
    (big $v:ty => f64, $x:ident) => {
        Value::F64(get_param::<$v>($x, 0)?.to_f64()?)
    };
    (big $v:ty => f32, $x:ident) => {
        Value::F32(get_param::<$v>($x, 0)?.to_f32()?)
    };
    (big $v:ty => f16, $x:ident) => {
        Value::F16(f16::from_f64(get_param::<$v>($x, 0)?.to_f64()?))
    };
    ($v:ty => $vo:ident, $x:ident) => {
        Value::$vo(get_param::<$v>($x, 0)?.try_into().ok()?)
    };
}

#[macro_export]
macro_rules! typecast_float {
    ($v:ty => str, $x:ident) => {
        Value::Str(get_param::<$v>($x, 0)?.to_string())
    };
    ($v:ty => bool, $x:ident) => {
        Value::Bool(get_param::<$v>($x, 0)? == 0.0.into())
    };
    (f32 => f16, $x:ident) => {
        Value::F16(f16::from_f32(get_param::<f32>($x, 0)?))
    };
    (f64 => f16, $x:ident) => {
        Value::F16(f16::from_f64(get_param::<f64>($x, 0)?))
    };
    ($v:ty => $vo:ident $f:ident, $x:ident) => {
        Value::$vo(get_param::<$v>($x, 0)?.$f()?)
    };
}

pub fn arith_opr<'a, T: ValueInner>(
    h: &mut HashMap<&'a str, Value>,
    n: &'a str,
    f: &'static (dyn Fn(T, T) -> T + Send + Sync),
    this_ty: Arc<Type>,
) {
    binary(
        h,
        n,
        Arc::new(|x: &Vec<Value>| Some(f(get_param::<T>(&x, 0)?, get_param::<T>(x, 1)?).into())),
        Arc::clone(&this_ty),
        Arc::clone(&this_ty),
        Arc::clone(&this_ty),
    )
}

pub fn arith_opr_op<'a, T: ValueInner>(
    h: &mut HashMap<&'a str, Value>,
    n: &'a str,
    f: &'static (dyn Fn(&T, &T) -> Option<T> + Send + Sync),
    this_ty: Arc<Type>,
) {
    binary(
        h,
        n,
        Arc::new(|x: &Vec<Value>| Some(f(&get_param::<T>(&x, 0)?, &get_param::<T>(x, 1)?)?.into())),
        Arc::clone(&this_ty),
        Arc::clone(&this_ty),
        Arc::clone(&this_ty),
    )
}

pub fn arith_opr_default<
    T: CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + CheckedRem + ValueInner,
>(
    h: &mut HashMap<&str, Value>,
    this_ty: Arc<Type>,
) {
    arith_opr_op(h, "_add", &T::checked_add, Arc::clone(&this_ty));
    arith_opr_op(h, "_sub", &T::checked_sub, Arc::clone(&this_ty));
    arith_opr_op(h, "_mul", &T::checked_mul, Arc::clone(&this_ty));
    arith_opr_op(h, "_div", &T::checked_div, Arc::clone(&this_ty));
    arith_opr_op(h, "_rem", &T::checked_rem, Arc::clone(&this_ty));
}

pub fn arith_opr_big_default<
    T: CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + Rem<T> + ValueInner,
>(
    h: &mut HashMap<&str, Value>,
    this_ty: Arc<Type>,
) {
    arith_opr_op(h, "_add", &T::checked_add, Arc::clone(&this_ty));
    arith_opr_op(h, "_sub", &T::checked_sub, Arc::clone(&this_ty));
    arith_opr_op(h, "_mul", &T::checked_mul, Arc::clone(&this_ty));
    arith_opr_op(h, "_div", &T::checked_div, Arc::clone(&this_ty));
}

pub fn arith_opr_float_default<T: Float + ValueInner>(
    h: &mut HashMap<&str, Value>,
    this_ty: Arc<Type>,
) {
    arith_opr(h, "_add", &Add::<T>::add, Arc::clone(&this_ty));
    arith_opr(h, "_sub", &Sub::<T>::sub, Arc::clone(&this_ty));
    arith_opr(h, "_mul", &Mul::<T>::mul, Arc::clone(&this_ty));
    arith_opr(h, "_div", &Div::<T>::div, Arc::clone(&this_ty));
    arith_opr(h, "_rem", &Rem::<T>::rem, Arc::clone(&this_ty));
}

pub fn comp_opr<'a, T: ValueInner>(
    h: &mut HashMap<&'a str, Value>,
    n: &'a str,
    f: &'static (dyn Fn(&T, &T) -> bool + Send + Sync),
    this_ty: Arc<Type>,
) {
    binary(
        h,
        n,
        Arc::new(|x: &Vec<Value>| Some(f(&get_param::<T>(&x, 0)?, &get_param::<T>(x, 1)?).into())),
        Arc::clone(&this_ty),
        Arc::clone(&this_ty),
        Arc::clone(&BOOL_T),
    )
}

pub fn comp_opr_default<T: PartialOrd<T> + ValueInner>(
    h: &mut HashMap<&str, Value>,
    this_ty: Arc<Type>,
) {
    comp_opr(h, "_eq", &T::eq, Arc::clone(&this_ty));
    comp_opr(h, "_ne", &T::ne, Arc::clone(&this_ty));
    comp_opr(h, "_gt", &T::gt, Arc::clone(&this_ty));
    comp_opr(h, "_ge", &T::ge, Arc::clone(&this_ty));
    comp_opr(h, "_lt", &T::lt, Arc::clone(&this_ty));
    comp_opr(h, "_le", &T::le, Arc::clone(&this_ty));
}

pub fn concat(h: &mut HashMap<&str, Value>, this_ty: Arc<Type>) {
    binary(
        h,
        "_concat",
        Arc::new(|x: &Vec<Value>| Some(Value::Str(format!("{}{}", x[0], x[1])))),
        Arc::clone(&this_ty),
        Arc::clone(&ANY_T),
        Arc::clone(&STR_T),
    )
}

pub fn type_cast(h: &mut HashMap<&str, Value>, f: Arc<BuiltinFunction>, this_ty: Arc<Type>) {
    binary(
        h,
        "_typecast",
        f,
        Arc::clone(&this_ty),
        Arc::clone(&TYPE_T),
        Arc::clone(&ANY_T),
    )
}
