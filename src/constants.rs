// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLCompareFunction {
   Never = 0,
   Less = 1,
   Equal = 2,
   LessEqual = 3,
   Greater = 4,
   NotEqual = 5,
   GreaterEqual = 6,
   Always = 7
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLPixelFormat {
   Invalid                = 0,
   A8Unorm                = 1,
   R8Unorm                = 10,
   R8Snorm                = 12,
   R8Uint                 = 13,
   R8Sint                 = 14,
   R16Unorm               = 20,
   R16Snorm               = 22,
   R16Uint                = 23,
   R16Sint                = 24,
   R16Float               = 25,
   RG8Unorm               = 30,
   RG8Snorm               = 32,
   RG8Uint                = 33,
   RG8Sint                = 34,
   R32Uint                = 53,
   R32Sint                = 54,
   R32Float               = 55,
   RG16Unorm              = 60,
   RG16Snorm              = 62,
   RG16Uint               = 63,
   RG16Sint               = 64,
   RG16Float              = 65,
   RGBA8Unorm             = 70,
   RGBA8Unorm_sRGB        = 71,
   RGBA8Snorm             = 72,
   RGBA8Uint              = 73,
   RGBA8Sint              = 74,
   BGRA8Unorm             = 80,
   BGRA8Unorm_sRGB        = 81,
   RGB10A2Unorm           = 90,
   RGB10A2Uint            = 91,
   RG11B10Float           = 92,
   RGB9E5Float            = 93,
   RG32Uint               = 103,
   RG32Sint               = 104,
   RG32Float              = 105,
   RGBA16Unorm            = 110,
   RGBA16Snorm            = 112,
   RGBA16Uint             = 113,
   RGBA16Sint             = 114,
   RGBA16Float            = 115,
   RGBA32Uint             = 123,
   RGBA32Sint             = 124,
   RGBA32Float            = 125,
   BC1_RGBA               = 130,
   BC1_RGBA_sRGB          = 131,
   BC2_RGBA               = 132,
   BC2_RGBA_sRGB          = 133,
   BC3_RGBA               = 134,
   BC3_RGBA_sRGB          = 135,
   BC4_RUnorm             = 140,
   BC4_RSnorm             = 141,
   BC5_RGUnorm            = 142,
   BC5_RGSnorm            = 143,
   BC6H_RGBFloat          = 150,
   BC6H_RGBUfloat         = 151,
   BC7_RGBAUnorm          = 152,
   BC7_RGBAUnorm_sRGB     = 153,
   GBGR422                = 240,
   BGRG422                = 241,
   Depth32Float           = 252,
   Stencil8               = 253,
   Depth24Unorm_Stencil8  = 255,
   Depth32Float_Stencil8  = 260,
}
