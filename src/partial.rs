//////////////////////////////////////////
// This code was generated with Python. //
//////////////////////////////////////////



pub trait Partial1<T0, R>: Fn(T0) -> R + Copy {
    fn partial1(self, arg0: T0)
    -> impl Fn() -> R + Copy
    where T0: Copy;
}

impl<T0, R, F: Fn(T0) -> R + Copy>
Partial1<T0, R> for F {
    #[inline(always)]
    fn partial1(self, arg0: T0)
    -> impl Fn() -> R + Copy
    where T0: Copy {
        #[inline(always)]
        move || self(arg0)
    }
}
pub trait Partial2<T0, T1, R>: Fn(T0, T1) -> R + Copy {
    fn partial1(self, arg0: T0)
    -> impl Fn(T1) -> R + Copy
    where T0: Copy;
    fn partial2(self, arg0: T0, arg1: T1)
    -> impl Fn() -> R + Copy
    where
        T0: Copy,
        T1: Copy;
}

impl<T0, T1, R, F: Fn(T0, T1) -> R + Copy>
Partial2<T0, T1, R> for F {
    #[inline(always)]
    fn partial1(self, arg0: T0)
    -> impl Fn(T1) -> R + Copy
    where T0: Copy {
        #[inline(always)]
        move |arg1: T1| self(arg0, arg1)
    }

    #[inline(always)]
    fn partial2(self, arg0: T0, arg1: T1)
    -> impl Fn() -> R + Copy
    where
        T0: Copy,
        T1: Copy {
        #[inline(always)]
        move || self(arg0, arg1)
    }
}
pub trait Partial3<T0, T1, T2, R>: Fn(T0, T1, T2) -> R + Copy {
    fn partial1(self, arg0: T0)
    -> impl Fn(T1, T2) -> R + Copy
    where T0: Copy;
    fn partial2(self, arg0: T0, arg1: T1)
    -> impl Fn(T2) -> R + Copy
    where
        T0: Copy,
        T1: Copy;
    fn partial3(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn() -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy;
}

impl<T0, T1, T2, R, F: Fn(T0, T1, T2) -> R + Copy>
Partial3<T0, T1, T2, R> for F {
    #[inline(always)]
    fn partial1(self, arg0: T0)
    -> impl Fn(T1, T2) -> R + Copy
    where T0: Copy {
        #[inline(always)]
        move |arg1: T1, arg2: T2| self(arg0, arg1, arg2)
    }

    #[inline(always)]
    fn partial2(self, arg0: T0, arg1: T1)
    -> impl Fn(T2) -> R + Copy
    where
        T0: Copy,
        T1: Copy {
        #[inline(always)]
        move |arg2: T2| self(arg0, arg1, arg2)
    }

    #[inline(always)]
    fn partial3(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn() -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy {
        #[inline(always)]
        move || self(arg0, arg1, arg2)
    }
}
pub trait Partial4<T0, T1, T2, T3, R>: Fn(T0, T1, T2, T3) -> R + Copy {
    fn partial1(self, arg0: T0)
    -> impl Fn(T1, T2, T3) -> R + Copy
    where T0: Copy;
    fn partial2(self, arg0: T0, arg1: T1)
    -> impl Fn(T2, T3) -> R + Copy
    where
        T0: Copy,
        T1: Copy;
    fn partial3(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn(T3) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy;
    fn partial4(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl Fn() -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy;
}

impl<T0, T1, T2, T3, R, F: Fn(T0, T1, T2, T3) -> R + Copy>
Partial4<T0, T1, T2, T3, R> for F {
    #[inline(always)]
    fn partial1(self, arg0: T0)
    -> impl Fn(T1, T2, T3) -> R + Copy
    where T0: Copy {
        #[inline(always)]
        move |arg1: T1, arg2: T2, arg3: T3| self(arg0, arg1, arg2, arg3)
    }

    #[inline(always)]
    fn partial2(self, arg0: T0, arg1: T1)
    -> impl Fn(T2, T3) -> R + Copy
    where
        T0: Copy,
        T1: Copy {
        #[inline(always)]
        move |arg2: T2, arg3: T3| self(arg0, arg1, arg2, arg3)
    }

    #[inline(always)]
    fn partial3(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn(T3) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy {
        #[inline(always)]
        move |arg3: T3| self(arg0, arg1, arg2, arg3)
    }

    #[inline(always)]
    fn partial4(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl Fn() -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy {
        #[inline(always)]
        move || self(arg0, arg1, arg2, arg3)
    }
}
pub trait Partial5<T0, T1, T2, T3, T4, R>: Fn(T0, T1, T2, T3, T4) -> R + Copy {
    fn partial1(self, arg0: T0)
    -> impl Fn(T1, T2, T3, T4) -> R + Copy
    where T0: Copy;
    fn partial2(self, arg0: T0, arg1: T1)
    -> impl Fn(T2, T3, T4) -> R + Copy
    where
        T0: Copy,
        T1: Copy;
    fn partial3(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn(T3, T4) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy;
    fn partial4(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl Fn(T4) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy;
    fn partial5(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl Fn() -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy,
        T4: Copy;
}

impl<T0, T1, T2, T3, T4, R, F: Fn(T0, T1, T2, T3, T4) -> R + Copy>
Partial5<T0, T1, T2, T3, T4, R> for F {
    #[inline(always)]
    fn partial1(self, arg0: T0)
    -> impl Fn(T1, T2, T3, T4) -> R + Copy
    where T0: Copy {
        #[inline(always)]
        move |arg1: T1, arg2: T2, arg3: T3, arg4: T4| self(arg0, arg1, arg2, arg3, arg4)
    }

    #[inline(always)]
    fn partial2(self, arg0: T0, arg1: T1)
    -> impl Fn(T2, T3, T4) -> R + Copy
    where
        T0: Copy,
        T1: Copy {
        #[inline(always)]
        move |arg2: T2, arg3: T3, arg4: T4| self(arg0, arg1, arg2, arg3, arg4)
    }

    #[inline(always)]
    fn partial3(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn(T3, T4) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy {
        #[inline(always)]
        move |arg3: T3, arg4: T4| self(arg0, arg1, arg2, arg3, arg4)
    }

    #[inline(always)]
    fn partial4(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl Fn(T4) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy {
        #[inline(always)]
        move |arg4: T4| self(arg0, arg1, arg2, arg3, arg4)
    }

    #[inline(always)]
    fn partial5(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl Fn() -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy,
        T4: Copy {
        #[inline(always)]
        move || self(arg0, arg1, arg2, arg3, arg4)
    }
}
pub trait Partial6<T0, T1, T2, T3, T4, T5, R>: Fn(T0, T1, T2, T3, T4, T5) -> R + Copy {
    fn partial1(self, arg0: T0)
    -> impl Fn(T1, T2, T3, T4, T5) -> R + Copy
    where T0: Copy;
    fn partial2(self, arg0: T0, arg1: T1)
    -> impl Fn(T2, T3, T4, T5) -> R + Copy
    where
        T0: Copy,
        T1: Copy;
    fn partial3(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn(T3, T4, T5) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy;
    fn partial4(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl Fn(T4, T5) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy;
    fn partial5(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl Fn(T5) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy,
        T4: Copy;
    fn partial6(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5)
    -> impl Fn() -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy,
        T4: Copy,
        T5: Copy;
}

impl<T0, T1, T2, T3, T4, T5, R, F: Fn(T0, T1, T2, T3, T4, T5) -> R + Copy>
Partial6<T0, T1, T2, T3, T4, T5, R> for F {
    #[inline(always)]
    fn partial1(self, arg0: T0)
    -> impl Fn(T1, T2, T3, T4, T5) -> R + Copy
    where T0: Copy {
        #[inline(always)]
        move |arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5| self(arg0, arg1, arg2, arg3, arg4, arg5)
    }

    #[inline(always)]
    fn partial2(self, arg0: T0, arg1: T1)
    -> impl Fn(T2, T3, T4, T5) -> R + Copy
    where
        T0: Copy,
        T1: Copy {
        #[inline(always)]
        move |arg2: T2, arg3: T3, arg4: T4, arg5: T5| self(arg0, arg1, arg2, arg3, arg4, arg5)
    }

    #[inline(always)]
    fn partial3(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn(T3, T4, T5) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy {
        #[inline(always)]
        move |arg3: T3, arg4: T4, arg5: T5| self(arg0, arg1, arg2, arg3, arg4, arg5)
    }

    #[inline(always)]
    fn partial4(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl Fn(T4, T5) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy {
        #[inline(always)]
        move |arg4: T4, arg5: T5| self(arg0, arg1, arg2, arg3, arg4, arg5)
    }

    #[inline(always)]
    fn partial5(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl Fn(T5) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy,
        T4: Copy {
        #[inline(always)]
        move |arg5: T5| self(arg0, arg1, arg2, arg3, arg4, arg5)
    }

    #[inline(always)]
    fn partial6(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5)
    -> impl Fn() -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy,
        T4: Copy,
        T5: Copy {
        #[inline(always)]
        move || self(arg0, arg1, arg2, arg3, arg4, arg5)
    }
}
pub trait Partial7<T0, T1, T2, T3, T4, T5, T6, R>: Fn(T0, T1, T2, T3, T4, T5, T6) -> R + Copy {
    fn partial1(self, arg0: T0)
    -> impl Fn(T1, T2, T3, T4, T5, T6) -> R + Copy
    where T0: Copy;
    fn partial2(self, arg0: T0, arg1: T1)
    -> impl Fn(T2, T3, T4, T5, T6) -> R + Copy
    where
        T0: Copy,
        T1: Copy;
    fn partial3(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn(T3, T4, T5, T6) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy;
    fn partial4(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl Fn(T4, T5, T6) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy;
    fn partial5(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl Fn(T5, T6) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy,
        T4: Copy;
    fn partial6(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5)
    -> impl Fn(T6) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy,
        T4: Copy,
        T5: Copy;
    fn partial7(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6)
    -> impl Fn() -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy,
        T4: Copy,
        T5: Copy,
        T6: Copy;
}

impl<T0, T1, T2, T3, T4, T5, T6, R, F: Fn(T0, T1, T2, T3, T4, T5, T6) -> R + Copy>
Partial7<T0, T1, T2, T3, T4, T5, T6, R> for F {
    #[inline(always)]
    fn partial1(self, arg0: T0)
    -> impl Fn(T1, T2, T3, T4, T5, T6) -> R + Copy
    where T0: Copy {
        #[inline(always)]
        move |arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
    }

    #[inline(always)]
    fn partial2(self, arg0: T0, arg1: T1)
    -> impl Fn(T2, T3, T4, T5, T6) -> R + Copy
    where
        T0: Copy,
        T1: Copy {
        #[inline(always)]
        move |arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
    }

    #[inline(always)]
    fn partial3(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn(T3, T4, T5, T6) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy {
        #[inline(always)]
        move |arg3: T3, arg4: T4, arg5: T5, arg6: T6| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
    }

    #[inline(always)]
    fn partial4(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl Fn(T4, T5, T6) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy {
        #[inline(always)]
        move |arg4: T4, arg5: T5, arg6: T6| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
    }

    #[inline(always)]
    fn partial5(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl Fn(T5, T6) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy,
        T4: Copy {
        #[inline(always)]
        move |arg5: T5, arg6: T6| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
    }

    #[inline(always)]
    fn partial6(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5)
    -> impl Fn(T6) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy,
        T4: Copy,
        T5: Copy {
        #[inline(always)]
        move |arg6: T6| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
    }

    #[inline(always)]
    fn partial7(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6)
    -> impl Fn() -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy,
        T4: Copy,
        T5: Copy,
        T6: Copy {
        #[inline(always)]
        move || self(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
    }
}
pub trait Partial8<T0, T1, T2, T3, T4, T5, T6, T7, R>: Fn(T0, T1, T2, T3, T4, T5, T6, T7) -> R + Copy {
    fn partial1(self, arg0: T0)
    -> impl Fn(T1, T2, T3, T4, T5, T6, T7) -> R + Copy
    where T0: Copy;
    fn partial2(self, arg0: T0, arg1: T1)
    -> impl Fn(T2, T3, T4, T5, T6, T7) -> R + Copy
    where
        T0: Copy,
        T1: Copy;
    fn partial3(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn(T3, T4, T5, T6, T7) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy;
    fn partial4(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl Fn(T4, T5, T6, T7) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy;
    fn partial5(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl Fn(T5, T6, T7) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy,
        T4: Copy;
    fn partial6(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5)
    -> impl Fn(T6, T7) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy,
        T4: Copy,
        T5: Copy;
    fn partial7(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6)
    -> impl Fn(T7) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy,
        T4: Copy,
        T5: Copy,
        T6: Copy;
    fn partial8(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6, arg7: T7)
    -> impl Fn() -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy,
        T4: Copy,
        T5: Copy,
        T6: Copy,
        T7: Copy;
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, R, F: Fn(T0, T1, T2, T3, T4, T5, T6, T7) -> R + Copy>
Partial8<T0, T1, T2, T3, T4, T5, T6, T7, R> for F {
    #[inline(always)]
    fn partial1(self, arg0: T0)
    -> impl Fn(T1, T2, T3, T4, T5, T6, T7) -> R + Copy
    where T0: Copy {
        #[inline(always)]
        move |arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6, arg7: T7| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
    }

    #[inline(always)]
    fn partial2(self, arg0: T0, arg1: T1)
    -> impl Fn(T2, T3, T4, T5, T6, T7) -> R + Copy
    where
        T0: Copy,
        T1: Copy {
        #[inline(always)]
        move |arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6, arg7: T7| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
    }

    #[inline(always)]
    fn partial3(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn(T3, T4, T5, T6, T7) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy {
        #[inline(always)]
        move |arg3: T3, arg4: T4, arg5: T5, arg6: T6, arg7: T7| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
    }

    #[inline(always)]
    fn partial4(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl Fn(T4, T5, T6, T7) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy {
        #[inline(always)]
        move |arg4: T4, arg5: T5, arg6: T6, arg7: T7| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
    }

    #[inline(always)]
    fn partial5(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl Fn(T5, T6, T7) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy,
        T4: Copy {
        #[inline(always)]
        move |arg5: T5, arg6: T6, arg7: T7| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
    }

    #[inline(always)]
    fn partial6(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5)
    -> impl Fn(T6, T7) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy,
        T4: Copy,
        T5: Copy {
        #[inline(always)]
        move |arg6: T6, arg7: T7| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
    }

    #[inline(always)]
    fn partial7(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6)
    -> impl Fn(T7) -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy,
        T4: Copy,
        T5: Copy,
        T6: Copy {
        #[inline(always)]
        move |arg7: T7| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
    }

    #[inline(always)]
    fn partial8(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6, arg7: T7)
    -> impl Fn() -> R + Copy
    where
        T0: Copy,
        T1: Copy,
        T2: Copy,
        T3: Copy,
        T4: Copy,
        T5: Copy,
        T6: Copy,
        T7: Copy {
        #[inline(always)]
        move || self(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
    }
}