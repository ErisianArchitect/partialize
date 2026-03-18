//////////////////////////////////////////
// This code was generated with Python. //
//////////////////////////////////////////



pub trait Partial1Once<T0, R>: FnOnce(T0) -> R {
    fn partial1_once(self, arg0: T0)
    -> impl FnOnce() -> R;
}

impl<T0, R, F: FnOnce(T0) -> R>
Partial1Once<T0, R> for F {
    #[inline(always)]
    fn partial1_once(self, arg0: T0)
    -> impl FnOnce() -> R {
        #[inline(always)]
        move || self(arg0)
    }
}
pub trait Partial2Once<T0, T1, R>: FnOnce(T0, T1) -> R {
    fn partial1_once(self, arg0: T0)
    -> impl FnOnce(T1) -> R;
    fn partial2_once(self, arg0: T0, arg1: T1)
    -> impl FnOnce() -> R;
}

impl<T0, T1, R, F: FnOnce(T0, T1) -> R>
Partial2Once<T0, T1, R> for F {
    #[inline(always)]
    fn partial1_once(self, arg0: T0)
    -> impl FnOnce(T1) -> R {
        #[inline(always)]
        move |arg1: T1| self(arg0, arg1)
    }

    #[inline(always)]
    fn partial2_once(self, arg0: T0, arg1: T1)
    -> impl FnOnce() -> R {
        #[inline(always)]
        move || self(arg0, arg1)
    }
}
pub trait Partial3Once<T0, T1, T2, R>: FnOnce(T0, T1, T2) -> R {
    fn partial1_once(self, arg0: T0)
    -> impl FnOnce(T1, T2) -> R;
    fn partial2_once(self, arg0: T0, arg1: T1)
    -> impl FnOnce(T2) -> R;
    fn partial3_once(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl FnOnce() -> R;
}

impl<T0, T1, T2, R, F: FnOnce(T0, T1, T2) -> R>
Partial3Once<T0, T1, T2, R> for F {
    #[inline(always)]
    fn partial1_once(self, arg0: T0)
    -> impl FnOnce(T1, T2) -> R {
        #[inline(always)]
        move |arg1: T1, arg2: T2| self(arg0, arg1, arg2)
    }

    #[inline(always)]
    fn partial2_once(self, arg0: T0, arg1: T1)
    -> impl FnOnce(T2) -> R {
        #[inline(always)]
        move |arg2: T2| self(arg0, arg1, arg2)
    }

    #[inline(always)]
    fn partial3_once(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl FnOnce() -> R {
        #[inline(always)]
        move || self(arg0, arg1, arg2)
    }
}
pub trait Partial4Once<T0, T1, T2, T3, R>: FnOnce(T0, T1, T2, T3) -> R {
    fn partial1_once(self, arg0: T0)
    -> impl FnOnce(T1, T2, T3) -> R;
    fn partial2_once(self, arg0: T0, arg1: T1)
    -> impl FnOnce(T2, T3) -> R;
    fn partial3_once(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl FnOnce(T3) -> R;
    fn partial4_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl FnOnce() -> R;
}

impl<T0, T1, T2, T3, R, F: FnOnce(T0, T1, T2, T3) -> R>
Partial4Once<T0, T1, T2, T3, R> for F {
    #[inline(always)]
    fn partial1_once(self, arg0: T0)
    -> impl FnOnce(T1, T2, T3) -> R {
        #[inline(always)]
        move |arg1: T1, arg2: T2, arg3: T3| self(arg0, arg1, arg2, arg3)
    }

    #[inline(always)]
    fn partial2_once(self, arg0: T0, arg1: T1)
    -> impl FnOnce(T2, T3) -> R {
        #[inline(always)]
        move |arg2: T2, arg3: T3| self(arg0, arg1, arg2, arg3)
    }

    #[inline(always)]
    fn partial3_once(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl FnOnce(T3) -> R {
        #[inline(always)]
        move |arg3: T3| self(arg0, arg1, arg2, arg3)
    }

    #[inline(always)]
    fn partial4_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl FnOnce() -> R {
        #[inline(always)]
        move || self(arg0, arg1, arg2, arg3)
    }
}
pub trait Partial5Once<T0, T1, T2, T3, T4, R>: FnOnce(T0, T1, T2, T3, T4) -> R {
    fn partial1_once(self, arg0: T0)
    -> impl FnOnce(T1, T2, T3, T4) -> R;
    fn partial2_once(self, arg0: T0, arg1: T1)
    -> impl FnOnce(T2, T3, T4) -> R;
    fn partial3_once(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl FnOnce(T3, T4) -> R;
    fn partial4_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl FnOnce(T4) -> R;
    fn partial5_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl FnOnce() -> R;
}

impl<T0, T1, T2, T3, T4, R, F: FnOnce(T0, T1, T2, T3, T4) -> R>
Partial5Once<T0, T1, T2, T3, T4, R> for F {
    #[inline(always)]
    fn partial1_once(self, arg0: T0)
    -> impl FnOnce(T1, T2, T3, T4) -> R {
        #[inline(always)]
        move |arg1: T1, arg2: T2, arg3: T3, arg4: T4| self(arg0, arg1, arg2, arg3, arg4)
    }

    #[inline(always)]
    fn partial2_once(self, arg0: T0, arg1: T1)
    -> impl FnOnce(T2, T3, T4) -> R {
        #[inline(always)]
        move |arg2: T2, arg3: T3, arg4: T4| self(arg0, arg1, arg2, arg3, arg4)
    }

    #[inline(always)]
    fn partial3_once(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl FnOnce(T3, T4) -> R {
        #[inline(always)]
        move |arg3: T3, arg4: T4| self(arg0, arg1, arg2, arg3, arg4)
    }

    #[inline(always)]
    fn partial4_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl FnOnce(T4) -> R {
        #[inline(always)]
        move |arg4: T4| self(arg0, arg1, arg2, arg3, arg4)
    }

    #[inline(always)]
    fn partial5_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl FnOnce() -> R {
        #[inline(always)]
        move || self(arg0, arg1, arg2, arg3, arg4)
    }
}
pub trait Partial6Once<T0, T1, T2, T3, T4, T5, R>: FnOnce(T0, T1, T2, T3, T4, T5) -> R {
    fn partial1_once(self, arg0: T0)
    -> impl FnOnce(T1, T2, T3, T4, T5) -> R;
    fn partial2_once(self, arg0: T0, arg1: T1)
    -> impl FnOnce(T2, T3, T4, T5) -> R;
    fn partial3_once(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl FnOnce(T3, T4, T5) -> R;
    fn partial4_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl FnOnce(T4, T5) -> R;
    fn partial5_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl FnOnce(T5) -> R;
    fn partial6_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5)
    -> impl FnOnce() -> R;
}

impl<T0, T1, T2, T3, T4, T5, R, F: FnOnce(T0, T1, T2, T3, T4, T5) -> R>
Partial6Once<T0, T1, T2, T3, T4, T5, R> for F {
    #[inline(always)]
    fn partial1_once(self, arg0: T0)
    -> impl FnOnce(T1, T2, T3, T4, T5) -> R {
        #[inline(always)]
        move |arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5| self(arg0, arg1, arg2, arg3, arg4, arg5)
    }

    #[inline(always)]
    fn partial2_once(self, arg0: T0, arg1: T1)
    -> impl FnOnce(T2, T3, T4, T5) -> R {
        #[inline(always)]
        move |arg2: T2, arg3: T3, arg4: T4, arg5: T5| self(arg0, arg1, arg2, arg3, arg4, arg5)
    }

    #[inline(always)]
    fn partial3_once(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl FnOnce(T3, T4, T5) -> R {
        #[inline(always)]
        move |arg3: T3, arg4: T4, arg5: T5| self(arg0, arg1, arg2, arg3, arg4, arg5)
    }

    #[inline(always)]
    fn partial4_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl FnOnce(T4, T5) -> R {
        #[inline(always)]
        move |arg4: T4, arg5: T5| self(arg0, arg1, arg2, arg3, arg4, arg5)
    }

    #[inline(always)]
    fn partial5_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl FnOnce(T5) -> R {
        #[inline(always)]
        move |arg5: T5| self(arg0, arg1, arg2, arg3, arg4, arg5)
    }

    #[inline(always)]
    fn partial6_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5)
    -> impl FnOnce() -> R {
        #[inline(always)]
        move || self(arg0, arg1, arg2, arg3, arg4, arg5)
    }
}
pub trait Partial7Once<T0, T1, T2, T3, T4, T5, T6, R>: FnOnce(T0, T1, T2, T3, T4, T5, T6) -> R {
    fn partial1_once(self, arg0: T0)
    -> impl FnOnce(T1, T2, T3, T4, T5, T6) -> R;
    fn partial2_once(self, arg0: T0, arg1: T1)
    -> impl FnOnce(T2, T3, T4, T5, T6) -> R;
    fn partial3_once(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl FnOnce(T3, T4, T5, T6) -> R;
    fn partial4_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl FnOnce(T4, T5, T6) -> R;
    fn partial5_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl FnOnce(T5, T6) -> R;
    fn partial6_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5)
    -> impl FnOnce(T6) -> R;
    fn partial7_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6)
    -> impl FnOnce() -> R;
}

impl<T0, T1, T2, T3, T4, T5, T6, R, F: FnOnce(T0, T1, T2, T3, T4, T5, T6) -> R>
Partial7Once<T0, T1, T2, T3, T4, T5, T6, R> for F {
    #[inline(always)]
    fn partial1_once(self, arg0: T0)
    -> impl FnOnce(T1, T2, T3, T4, T5, T6) -> R {
        #[inline(always)]
        move |arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
    }

    #[inline(always)]
    fn partial2_once(self, arg0: T0, arg1: T1)
    -> impl FnOnce(T2, T3, T4, T5, T6) -> R {
        #[inline(always)]
        move |arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
    }

    #[inline(always)]
    fn partial3_once(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl FnOnce(T3, T4, T5, T6) -> R {
        #[inline(always)]
        move |arg3: T3, arg4: T4, arg5: T5, arg6: T6| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
    }

    #[inline(always)]
    fn partial4_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl FnOnce(T4, T5, T6) -> R {
        #[inline(always)]
        move |arg4: T4, arg5: T5, arg6: T6| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
    }

    #[inline(always)]
    fn partial5_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl FnOnce(T5, T6) -> R {
        #[inline(always)]
        move |arg5: T5, arg6: T6| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
    }

    #[inline(always)]
    fn partial6_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5)
    -> impl FnOnce(T6) -> R {
        #[inline(always)]
        move |arg6: T6| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
    }

    #[inline(always)]
    fn partial7_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6)
    -> impl FnOnce() -> R {
        #[inline(always)]
        move || self(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
    }
}
pub trait Partial8Once<T0, T1, T2, T3, T4, T5, T6, T7, R>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7) -> R {
    fn partial1_once(self, arg0: T0)
    -> impl FnOnce(T1, T2, T3, T4, T5, T6, T7) -> R;
    fn partial2_once(self, arg0: T0, arg1: T1)
    -> impl FnOnce(T2, T3, T4, T5, T6, T7) -> R;
    fn partial3_once(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl FnOnce(T3, T4, T5, T6, T7) -> R;
    fn partial4_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl FnOnce(T4, T5, T6, T7) -> R;
    fn partial5_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl FnOnce(T5, T6, T7) -> R;
    fn partial6_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5)
    -> impl FnOnce(T6, T7) -> R;
    fn partial7_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6)
    -> impl FnOnce(T7) -> R;
    fn partial8_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6, arg7: T7)
    -> impl FnOnce() -> R;
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, R, F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7) -> R>
Partial8Once<T0, T1, T2, T3, T4, T5, T6, T7, R> for F {
    #[inline(always)]
    fn partial1_once(self, arg0: T0)
    -> impl FnOnce(T1, T2, T3, T4, T5, T6, T7) -> R {
        #[inline(always)]
        move |arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6, arg7: T7| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
    }

    #[inline(always)]
    fn partial2_once(self, arg0: T0, arg1: T1)
    -> impl FnOnce(T2, T3, T4, T5, T6, T7) -> R {
        #[inline(always)]
        move |arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6, arg7: T7| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
    }

    #[inline(always)]
    fn partial3_once(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl FnOnce(T3, T4, T5, T6, T7) -> R {
        #[inline(always)]
        move |arg3: T3, arg4: T4, arg5: T5, arg6: T6, arg7: T7| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
    }

    #[inline(always)]
    fn partial4_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl FnOnce(T4, T5, T6, T7) -> R {
        #[inline(always)]
        move |arg4: T4, arg5: T5, arg6: T6, arg7: T7| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
    }

    #[inline(always)]
    fn partial5_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl FnOnce(T5, T6, T7) -> R {
        #[inline(always)]
        move |arg5: T5, arg6: T6, arg7: T7| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
    }

    #[inline(always)]
    fn partial6_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5)
    -> impl FnOnce(T6, T7) -> R {
        #[inline(always)]
        move |arg6: T6, arg7: T7| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
    }

    #[inline(always)]
    fn partial7_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6)
    -> impl FnOnce(T7) -> R {
        #[inline(always)]
        move |arg7: T7| self(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
    }

    #[inline(always)]
    fn partial8_once(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6, arg7: T7)
    -> impl FnOnce() -> R {
        #[inline(always)]
        move || self(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
    }
}