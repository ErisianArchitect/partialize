//////////////////////////////////////////
// This code was generated with Python. //
//////////////////////////////////////////



pub trait Partial1Clone<T0, R>: Fn(T0) -> R + Clone {
    fn partial1_clone(self, arg0: T0)
    -> impl Fn() -> R + Clone
    where T0: Clone;
}

impl<T0, R, F: Fn(T0) -> R + Clone>
Partial1Clone<T0, R> for F {
    #[inline(always)]
    fn partial1_clone(self, arg0: T0)
    -> impl Fn() -> R + Clone
    where T0: Clone {
        #[inline(always)]
        move || self(arg0.clone())
    }
}
pub trait Partial2Clone<T0, T1, R>: Fn(T0, T1) -> R + Clone {
    fn partial1_clone(self, arg0: T0)
    -> impl Fn(T1) -> R + Clone
    where T0: Clone;
    fn partial2_clone(self, arg0: T0, arg1: T1)
    -> impl Fn() -> R + Clone
    where
        T0: Clone,
        T1: Clone;
}

impl<T0, T1, R, F: Fn(T0, T1) -> R + Clone>
Partial2Clone<T0, T1, R> for F {
    #[inline(always)]
    fn partial1_clone(self, arg0: T0)
    -> impl Fn(T1) -> R + Clone
    where T0: Clone {
        #[inline(always)]
        move |arg1: T1| self(arg0.clone(), arg1)
    }

    #[inline(always)]
    fn partial2_clone(self, arg0: T0, arg1: T1)
    -> impl Fn() -> R + Clone
    where
        T0: Clone,
        T1: Clone {
        #[inline(always)]
        move || self(arg0.clone(), arg1.clone())
    }
}
pub trait Partial3Clone<T0, T1, T2, R>: Fn(T0, T1, T2) -> R + Clone {
    fn partial1_clone(self, arg0: T0)
    -> impl Fn(T1, T2) -> R + Clone
    where T0: Clone;
    fn partial2_clone(self, arg0: T0, arg1: T1)
    -> impl Fn(T2) -> R + Clone
    where
        T0: Clone,
        T1: Clone;
    fn partial3_clone(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn() -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone;
}

impl<T0, T1, T2, R, F: Fn(T0, T1, T2) -> R + Clone>
Partial3Clone<T0, T1, T2, R> for F {
    #[inline(always)]
    fn partial1_clone(self, arg0: T0)
    -> impl Fn(T1, T2) -> R + Clone
    where T0: Clone {
        #[inline(always)]
        move |arg1: T1, arg2: T2| self(arg0.clone(), arg1, arg2)
    }

    #[inline(always)]
    fn partial2_clone(self, arg0: T0, arg1: T1)
    -> impl Fn(T2) -> R + Clone
    where
        T0: Clone,
        T1: Clone {
        #[inline(always)]
        move |arg2: T2| self(arg0.clone(), arg1.clone(), arg2)
    }

    #[inline(always)]
    fn partial3_clone(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn() -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone {
        #[inline(always)]
        move || self(arg0.clone(), arg1.clone(), arg2.clone())
    }
}
pub trait Partial4Clone<T0, T1, T2, T3, R>: Fn(T0, T1, T2, T3) -> R + Clone {
    fn partial1_clone(self, arg0: T0)
    -> impl Fn(T1, T2, T3) -> R + Clone
    where T0: Clone;
    fn partial2_clone(self, arg0: T0, arg1: T1)
    -> impl Fn(T2, T3) -> R + Clone
    where
        T0: Clone,
        T1: Clone;
    fn partial3_clone(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn(T3) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone;
    fn partial4_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl Fn() -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone;
}

impl<T0, T1, T2, T3, R, F: Fn(T0, T1, T2, T3) -> R + Clone>
Partial4Clone<T0, T1, T2, T3, R> for F {
    #[inline(always)]
    fn partial1_clone(self, arg0: T0)
    -> impl Fn(T1, T2, T3) -> R + Clone
    where T0: Clone {
        #[inline(always)]
        move |arg1: T1, arg2: T2, arg3: T3| self(arg0.clone(), arg1, arg2, arg3)
    }

    #[inline(always)]
    fn partial2_clone(self, arg0: T0, arg1: T1)
    -> impl Fn(T2, T3) -> R + Clone
    where
        T0: Clone,
        T1: Clone {
        #[inline(always)]
        move |arg2: T2, arg3: T3| self(arg0.clone(), arg1.clone(), arg2, arg3)
    }

    #[inline(always)]
    fn partial3_clone(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn(T3) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone {
        #[inline(always)]
        move |arg3: T3| self(arg0.clone(), arg1.clone(), arg2.clone(), arg3)
    }

    #[inline(always)]
    fn partial4_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl Fn() -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone {
        #[inline(always)]
        move || self(arg0.clone(), arg1.clone(), arg2.clone(), arg3.clone())
    }
}
pub trait Partial5Clone<T0, T1, T2, T3, T4, R>: Fn(T0, T1, T2, T3, T4) -> R + Clone {
    fn partial1_clone(self, arg0: T0)
    -> impl Fn(T1, T2, T3, T4) -> R + Clone
    where T0: Clone;
    fn partial2_clone(self, arg0: T0, arg1: T1)
    -> impl Fn(T2, T3, T4) -> R + Clone
    where
        T0: Clone,
        T1: Clone;
    fn partial3_clone(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn(T3, T4) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone;
    fn partial4_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl Fn(T4) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone;
    fn partial5_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl Fn() -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        T4: Clone;
}

impl<T0, T1, T2, T3, T4, R, F: Fn(T0, T1, T2, T3, T4) -> R + Clone>
Partial5Clone<T0, T1, T2, T3, T4, R> for F {
    #[inline(always)]
    fn partial1_clone(self, arg0: T0)
    -> impl Fn(T1, T2, T3, T4) -> R + Clone
    where T0: Clone {
        #[inline(always)]
        move |arg1: T1, arg2: T2, arg3: T3, arg4: T4| self(arg0.clone(), arg1, arg2, arg3, arg4)
    }

    #[inline(always)]
    fn partial2_clone(self, arg0: T0, arg1: T1)
    -> impl Fn(T2, T3, T4) -> R + Clone
    where
        T0: Clone,
        T1: Clone {
        #[inline(always)]
        move |arg2: T2, arg3: T3, arg4: T4| self(arg0.clone(), arg1.clone(), arg2, arg3, arg4)
    }

    #[inline(always)]
    fn partial3_clone(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn(T3, T4) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone {
        #[inline(always)]
        move |arg3: T3, arg4: T4| self(arg0.clone(), arg1.clone(), arg2.clone(), arg3, arg4)
    }

    #[inline(always)]
    fn partial4_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl Fn(T4) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone {
        #[inline(always)]
        move |arg4: T4| self(arg0.clone(), arg1.clone(), arg2.clone(), arg3.clone(), arg4)
    }

    #[inline(always)]
    fn partial5_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl Fn() -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        T4: Clone {
        #[inline(always)]
        move || self(arg0.clone(), arg1.clone(), arg2.clone(), arg3.clone(), arg4.clone())
    }
}
pub trait Partial6Clone<T0, T1, T2, T3, T4, T5, R>: Fn(T0, T1, T2, T3, T4, T5) -> R + Clone {
    fn partial1_clone(self, arg0: T0)
    -> impl Fn(T1, T2, T3, T4, T5) -> R + Clone
    where T0: Clone;
    fn partial2_clone(self, arg0: T0, arg1: T1)
    -> impl Fn(T2, T3, T4, T5) -> R + Clone
    where
        T0: Clone,
        T1: Clone;
    fn partial3_clone(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn(T3, T4, T5) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone;
    fn partial4_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl Fn(T4, T5) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone;
    fn partial5_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl Fn(T5) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        T4: Clone;
    fn partial6_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5)
    -> impl Fn() -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        T4: Clone,
        T5: Clone;
}

impl<T0, T1, T2, T3, T4, T5, R, F: Fn(T0, T1, T2, T3, T4, T5) -> R + Clone>
Partial6Clone<T0, T1, T2, T3, T4, T5, R> for F {
    #[inline(always)]
    fn partial1_clone(self, arg0: T0)
    -> impl Fn(T1, T2, T3, T4, T5) -> R + Clone
    where T0: Clone {
        #[inline(always)]
        move |arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5| self(arg0.clone(), arg1, arg2, arg3, arg4, arg5)
    }

    #[inline(always)]
    fn partial2_clone(self, arg0: T0, arg1: T1)
    -> impl Fn(T2, T3, T4, T5) -> R + Clone
    where
        T0: Clone,
        T1: Clone {
        #[inline(always)]
        move |arg2: T2, arg3: T3, arg4: T4, arg5: T5| self(arg0.clone(), arg1.clone(), arg2, arg3, arg4, arg5)
    }

    #[inline(always)]
    fn partial3_clone(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn(T3, T4, T5) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone {
        #[inline(always)]
        move |arg3: T3, arg4: T4, arg5: T5| self(arg0.clone(), arg1.clone(), arg2.clone(), arg3, arg4, arg5)
    }

    #[inline(always)]
    fn partial4_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl Fn(T4, T5) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone {
        #[inline(always)]
        move |arg4: T4, arg5: T5| self(arg0.clone(), arg1.clone(), arg2.clone(), arg3.clone(), arg4, arg5)
    }

    #[inline(always)]
    fn partial5_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl Fn(T5) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        T4: Clone {
        #[inline(always)]
        move |arg5: T5| self(arg0.clone(), arg1.clone(), arg2.clone(), arg3.clone(), arg4.clone(), arg5)
    }

    #[inline(always)]
    fn partial6_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5)
    -> impl Fn() -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        T4: Clone,
        T5: Clone {
        #[inline(always)]
        move || self(arg0.clone(), arg1.clone(), arg2.clone(), arg3.clone(), arg4.clone(), arg5.clone())
    }
}
pub trait Partial7Clone<T0, T1, T2, T3, T4, T5, T6, R>: Fn(T0, T1, T2, T3, T4, T5, T6) -> R + Clone {
    fn partial1_clone(self, arg0: T0)
    -> impl Fn(T1, T2, T3, T4, T5, T6) -> R + Clone
    where T0: Clone;
    fn partial2_clone(self, arg0: T0, arg1: T1)
    -> impl Fn(T2, T3, T4, T5, T6) -> R + Clone
    where
        T0: Clone,
        T1: Clone;
    fn partial3_clone(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn(T3, T4, T5, T6) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone;
    fn partial4_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl Fn(T4, T5, T6) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone;
    fn partial5_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl Fn(T5, T6) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        T4: Clone;
    fn partial6_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5)
    -> impl Fn(T6) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        T4: Clone,
        T5: Clone;
    fn partial7_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6)
    -> impl Fn() -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        T4: Clone,
        T5: Clone,
        T6: Clone;
}

impl<T0, T1, T2, T3, T4, T5, T6, R, F: Fn(T0, T1, T2, T3, T4, T5, T6) -> R + Clone>
Partial7Clone<T0, T1, T2, T3, T4, T5, T6, R> for F {
    #[inline(always)]
    fn partial1_clone(self, arg0: T0)
    -> impl Fn(T1, T2, T3, T4, T5, T6) -> R + Clone
    where T0: Clone {
        #[inline(always)]
        move |arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6| self(arg0.clone(), arg1, arg2, arg3, arg4, arg5, arg6)
    }

    #[inline(always)]
    fn partial2_clone(self, arg0: T0, arg1: T1)
    -> impl Fn(T2, T3, T4, T5, T6) -> R + Clone
    where
        T0: Clone,
        T1: Clone {
        #[inline(always)]
        move |arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6| self(arg0.clone(), arg1.clone(), arg2, arg3, arg4, arg5, arg6)
    }

    #[inline(always)]
    fn partial3_clone(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn(T3, T4, T5, T6) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone {
        #[inline(always)]
        move |arg3: T3, arg4: T4, arg5: T5, arg6: T6| self(arg0.clone(), arg1.clone(), arg2.clone(), arg3, arg4, arg5, arg6)
    }

    #[inline(always)]
    fn partial4_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl Fn(T4, T5, T6) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone {
        #[inline(always)]
        move |arg4: T4, arg5: T5, arg6: T6| self(arg0.clone(), arg1.clone(), arg2.clone(), arg3.clone(), arg4, arg5, arg6)
    }

    #[inline(always)]
    fn partial5_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl Fn(T5, T6) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        T4: Clone {
        #[inline(always)]
        move |arg5: T5, arg6: T6| self(arg0.clone(), arg1.clone(), arg2.clone(), arg3.clone(), arg4.clone(), arg5, arg6)
    }

    #[inline(always)]
    fn partial6_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5)
    -> impl Fn(T6) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        T4: Clone,
        T5: Clone {
        #[inline(always)]
        move |arg6: T6| self(arg0.clone(), arg1.clone(), arg2.clone(), arg3.clone(), arg4.clone(), arg5.clone(), arg6)
    }

    #[inline(always)]
    fn partial7_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6)
    -> impl Fn() -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        T4: Clone,
        T5: Clone,
        T6: Clone {
        #[inline(always)]
        move || self(arg0.clone(), arg1.clone(), arg2.clone(), arg3.clone(), arg4.clone(), arg5.clone(), arg6.clone())
    }
}
pub trait Partial8Clone<T0, T1, T2, T3, T4, T5, T6, T7, R>: Fn(T0, T1, T2, T3, T4, T5, T6, T7) -> R + Clone {
    fn partial1_clone(self, arg0: T0)
    -> impl Fn(T1, T2, T3, T4, T5, T6, T7) -> R + Clone
    where T0: Clone;
    fn partial2_clone(self, arg0: T0, arg1: T1)
    -> impl Fn(T2, T3, T4, T5, T6, T7) -> R + Clone
    where
        T0: Clone,
        T1: Clone;
    fn partial3_clone(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn(T3, T4, T5, T6, T7) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone;
    fn partial4_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl Fn(T4, T5, T6, T7) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone;
    fn partial5_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl Fn(T5, T6, T7) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        T4: Clone;
    fn partial6_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5)
    -> impl Fn(T6, T7) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        T4: Clone,
        T5: Clone;
    fn partial7_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6)
    -> impl Fn(T7) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        T4: Clone,
        T5: Clone,
        T6: Clone;
    fn partial8_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6, arg7: T7)
    -> impl Fn() -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        T4: Clone,
        T5: Clone,
        T6: Clone,
        T7: Clone;
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, R, F: Fn(T0, T1, T2, T3, T4, T5, T6, T7) -> R + Clone>
Partial8Clone<T0, T1, T2, T3, T4, T5, T6, T7, R> for F {
    #[inline(always)]
    fn partial1_clone(self, arg0: T0)
    -> impl Fn(T1, T2, T3, T4, T5, T6, T7) -> R + Clone
    where T0: Clone {
        #[inline(always)]
        move |arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6, arg7: T7| self(arg0.clone(), arg1, arg2, arg3, arg4, arg5, arg6, arg7)
    }

    #[inline(always)]
    fn partial2_clone(self, arg0: T0, arg1: T1)
    -> impl Fn(T2, T3, T4, T5, T6, T7) -> R + Clone
    where
        T0: Clone,
        T1: Clone {
        #[inline(always)]
        move |arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6, arg7: T7| self(arg0.clone(), arg1.clone(), arg2, arg3, arg4, arg5, arg6, arg7)
    }

    #[inline(always)]
    fn partial3_clone(self, arg0: T0, arg1: T1, arg2: T2)
    -> impl Fn(T3, T4, T5, T6, T7) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone {
        #[inline(always)]
        move |arg3: T3, arg4: T4, arg5: T5, arg6: T6, arg7: T7| self(arg0.clone(), arg1.clone(), arg2.clone(), arg3, arg4, arg5, arg6, arg7)
    }

    #[inline(always)]
    fn partial4_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3)
    -> impl Fn(T4, T5, T6, T7) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone {
        #[inline(always)]
        move |arg4: T4, arg5: T5, arg6: T6, arg7: T7| self(arg0.clone(), arg1.clone(), arg2.clone(), arg3.clone(), arg4, arg5, arg6, arg7)
    }

    #[inline(always)]
    fn partial5_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4)
    -> impl Fn(T5, T6, T7) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        T4: Clone {
        #[inline(always)]
        move |arg5: T5, arg6: T6, arg7: T7| self(arg0.clone(), arg1.clone(), arg2.clone(), arg3.clone(), arg4.clone(), arg5, arg6, arg7)
    }

    #[inline(always)]
    fn partial6_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5)
    -> impl Fn(T6, T7) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        T4: Clone,
        T5: Clone {
        #[inline(always)]
        move |arg6: T6, arg7: T7| self(arg0.clone(), arg1.clone(), arg2.clone(), arg3.clone(), arg4.clone(), arg5.clone(), arg6, arg7)
    }

    #[inline(always)]
    fn partial7_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6)
    -> impl Fn(T7) -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        T4: Clone,
        T5: Clone,
        T6: Clone {
        #[inline(always)]
        move |arg7: T7| self(arg0.clone(), arg1.clone(), arg2.clone(), arg3.clone(), arg4.clone(), arg5.clone(), arg6.clone(), arg7)
    }

    #[inline(always)]
    fn partial8_clone(self, arg0: T0, arg1: T1, arg2: T2, arg3: T3, arg4: T4, arg5: T5, arg6: T6, arg7: T7)
    -> impl Fn() -> R + Clone
    where
        T0: Clone,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        T4: Clone,
        T5: Clone,
        T6: Clone,
        T7: Clone {
        #[inline(always)]
        move || self(arg0.clone(), arg1.clone(), arg2.clone(), arg3.clone(), arg4.clone(), arg5.clone(), arg6.clone(), arg7.clone())
    }
}