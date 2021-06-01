use metron_core::{def_exp_scale};

pub mod scale_base {
    use metron_core::{def_exp_scale_base};
    def_exp_scale_base! {
        pub BP1024[f32 = 1024.0, f64 = 1024.0,],
        pub BP1000[f32 = 1000.0, f64 = 1000.0,],
        pub   BP10[f32 =   10.0, f64 =   10.0,],
        pub    BP2[f32 =    2.0, f64 =    2.0,],
    }
}
pub mod scale_exponent {
    use metron_core::{def_exp_scale_exp};
    def_exp_scale_exp! {
        pub   EP80[f32 =   80.0, f64 =   80.0,],
        pub   EP70[f32 =   70.0, f64 =   70.0,],
        pub   EP60[f32 =   60.0, f64 =   60.0,],
        pub   EP50[f32 =   50.0, f64 =   50.0,],
        pub   EP40[f32 =   40.0, f64 =   40.0,],
        pub   EP30[f32 =   30.0, f64 =   30.0,],
        pub   EP24[f32 =   24.0, f64 =   24.0,],
        pub   EP21[f32 =   21.0, f64 =   21.0,],
        pub   EP20[f32 =   20.0, f64 =   20.0,],
        pub   EP18[f32 =   18.0, f64 =   18.0,],
        pub   EP15[f32 =   15.0, f64 =   15.0,],
        pub   EP12[f32 =   12.0, f64 =   12.0,],
        pub   EP10[f32 =   10.0, f64 =   10.0,],
        pub    EP9[f32 =    9.0, f64 =    9.0,],
        pub    EP8[f32 =    8.0, f64 =    8.0,],
        pub    EP7[f32 =    7.0, f64 =    7.0,],
        pub    EP6[f32 =    6.0, f64 =    6.0,],
        pub    EP5[f32 =    5.0, f64 =    5.0,],
        pub    EP4[f32 =    4.0, f64 =    4.0,],
        pub    EP3[f32 =    3.0, f64 =    3.0,],
        pub    EP2[f32 =    2.0, f64 =    2.0,],
        pub    EP1[f32 =    1.0, f64 =    1.0,],
        pub     E0[f32 =    0.0, f64 =    0.0,],
        pub    EM1[f32 = -  1.0, f64 = -  1.0,],
        pub    EM2[f32 = -  2.0, f64 = -  2.0,],
        pub    EM3[f32 = -  3.0, f64 = -  3.0,],
        pub    EM4[f32 = -  4.0, f64 = -  4.0,],
        pub    EM5[f32 = -  5.0, f64 = -  5.0,],
        pub    EM6[f32 = -  6.0, f64 = -  6.0,],
        pub    EM7[f32 = -  7.0, f64 = -  7.0,],
        pub    EM8[f32 = -  8.0, f64 = -  8.0,],
        pub    EM9[f32 = -  9.0, f64 = -  9.0,],
        pub   EM12[f32 = - 12.0, f64 = - 12.0,],
        pub   EM15[f32 = - 15.0, f64 = - 15.0,],
        pub   EM18[f32 = - 18.0, f64 = - 18.0,],
        pub   EM21[f32 = - 21.0, f64 = - 21.0,],
        pub   EM24[f32 = - 24.0, f64 = - 24.0,],
    }
}

use scale_base::*;
use scale_exponent::*;
def_exp_scale! {
    pub One[
        BP2    =^ E0,
        BP10   =^ E0,
        BP1000 =^ E0,
    ] : default BP1000[f32,f32] ,
}
def_exp_scale!{
    pub  Yotta[ BP10 =^ EP24, BP1000 =^ EP8, ] : default BP1000[f32,f32],
    pub  Zetta[ BP10 =^ EP21, BP1000 =^ EP7, ] : default BP1000[f32,f32],
    pub    Exa[ BP10 =^ EP18, BP1000 =^ EP6, ] : default BP1000[f32,f32],
    pub   Peta[ BP10 =^ EP15, BP1000 =^ EP5, ] : default BP1000[f32,f32],
    pub   Tera[ BP10 =^ EP12, BP1000 =^ EP4, ] : default BP1000[f32,f32],
    pub   Giga[ BP10 =^  EP9, BP1000 =^ EP3, ] : default BP1000[f32,f32],
    pub   Mega[ BP10 =^  EP6, BP1000 =^ EP2, ] : default BP1000[f32,f32],
    pub   Kilo[ BP10 =^  EP3, BP1000 =^ EP1, ] : default BP1000[f32,f32],
    pub  Hecto[ BP10 =^  EP2,                ]                          ,
    pub   Deka[ BP10 =^  EP1,                ]                          ,
    pub   Deci[ BP10 =^  EM1,                ]                          ,
    pub  Centi[ BP10 =^  EM2,                ]                          ,
    pub  Milli[ BP10 =^  EM3, BP1000 =^ EM1, ] : default BP1000[f32,f32],
    pub  Micro[ BP10 =^  EM6, BP1000 =^ EM2, ] : default BP1000[f32,f32],
    pub   Nano[ BP10 =^  EM9, BP1000 =^ EM3, ] : default BP1000[f32,f32],
    pub   Pico[ BP10 =^ EM12, BP1000 =^ EM4, ] : default BP1000[f32,f32],
    pub  Femto[ BP10 =^ EM15, BP1000 =^ EM5, ] : default BP1000[f32,f32],
    pub   Atto[ BP10 =^ EM18, BP1000 =^ EM6, ] : default BP1000[f32,f32],
    pub  Zepto[ BP10 =^ EM21, BP1000 =^ EM7, ] : default BP1000[f32,f32],
    pub  Yocto[ BP10 =^ EM24, BP1000 =^ EM8, ] : default BP1000[f32,f32],
}
def_exp_scale!{
    pub   Yobi[ BP2 =^ EP80, BP1024 =^ EP8, ] : default BP1024[f32,f32],
    pub   Zebi[ BP2 =^ EP70, BP1024 =^ EP7, ] : default BP1024[f32,f32],
    pub   Exbi[ BP2 =^ EP60, BP1024 =^ EP6, ] : default BP1024[f32,f32],
    pub   Pebi[ BP2 =^ EP50, BP1024 =^ EP5, ] : default BP1024[f32,f32],
    pub   Tebi[ BP2 =^ EP40, BP1024 =^ EP4, ] : default BP1024[f32,f32],
    pub   Gibi[ BP2 =^ EP30, BP1024 =^ EP3, ] : default BP1024[f32,f32],
    pub   Mebi[ BP2 =^ EP20, BP1024 =^ EP2, ] : default BP1024[f32,f32],
    pub   Kibi[ BP2 =^ EP10, BP1024 =^ EP1, ] : default BP1024[f32,f32],
}