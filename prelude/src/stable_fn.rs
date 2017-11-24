pub trait StableFnOnce<Args> {
    type Output;
    fn call_once(self, args: Args) -> Self::Output;
}

pub trait StableFnMut<Args>: StableFnOnce<Args> {
    fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait StableFn<Args>: StableFnMut<Args> {
    fn call(&self, args: Args) -> Self::Output;
}

macro_rules! impl_for_kind {
    ($($name:ident),*) => {
        impl <$($name,)* R,F:FnOnce($($name,)*)->R> StableFnOnce<($($name,)*)> for F {
            type Output = R;
            #[allow(non_snake_case)]
            fn call_once(self, ($($name,)*): ($($name,)*)) -> R {
                self($($name,)*)
            }
        }
        impl <$($name,)* R,F:FnMut($($name,)*)->R> StableFnMut<($($name,)*)> for F {
            #[allow(non_snake_case)]
            fn call_mut(&mut self, ($($name,)*): ($($name,)*)) -> R {
                self($($name,)*)
            }
        }
        impl <$($name,)* R,F:Fn($($name,)*)->R> StableFn<($($name,)*)> for F {
            #[allow(non_snake_case)]
            fn call(&self, ($($name,)*): ($($name,)*)) -> R {
                self($($name,)*)
            }
        }
    }
}

impl_for_kind!();
impl_for_kind!(A);
impl_for_kind!(A, B);
impl_for_kind!(A, B, C);
impl_for_kind!(A, B, C, D);