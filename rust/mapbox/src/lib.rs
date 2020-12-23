use mapbox_sys as sys;

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum RunLoopType {
    New,
    Default,
}

pub struct Map {
    lraw: *mut sys::mbgl_run_loop_t,
    raw: *mut sys::mbgl_map_t,
    render_retries: i32,
}

impl Drop for Map {
    fn drop(&mut self) {
        unsafe {
            sys::mbgl_map_free(self.raw);
            sys::mbgl_run_loop_free(self.lraw);
        }
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MapOptions<'a> {
    pub size: Size,
    pub pixel_ratio: PixelRatio,
    pub access_token: Option<&'a str>,
    pub cache_path: Option<&'a str>,
    pub assets_path: Option<&'a str>,
    pub render_retries: i32,
}

impl<'a> Default for MapOptions<'a> {
    fn default() -> MapOptions<'a> {
        MapOptions {
            size: Size(512, 512),
            pixel_ratio: PixelRatio::Normal,
            access_token: None,
            cache_path: None,
            assets_path: None,
            render_retries: 5,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PixelRatio {
    Normal,
    Retina,
}

impl Default for PixelRatio {
    fn default() -> Self {
        PixelRatio::Normal
    }
}

impl PixelRatio {
    fn as_int(&self) -> i32 {
        match self {
            PixelRatio::Normal => 1,
            PixelRatio::Retina => 2,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LatLng(pub f64, pub f64);

#[derive(Clone, Copy, PartialEq, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Size(pub u32, pub u32);

impl Size {
    pub const MAX: Size = Size(2000, 2000);
    pub const MIN: Size = Size(64, 64);
    pub fn is_valid(&self) -> bool {
        self.0 >= Self::MIN.0
            && self.0 <= Self::MAX.0
            && self.1 >= Self::MIN.1
            && self.1 <= Self::MAX.1
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, PartialEq)]
pub struct JumpToOptions {
    pub center: LatLng,
    pub zoom: Option<f64>,
}

impl Map {
    fn try_render(&self, retries: i32) -> Option<image::RgbaImage> {
        unsafe {
            let buffer = sys::mbgl_map_render(self.raw);
            if buffer == std::ptr::null_mut() {
                if retries > 0 {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    return self.try_render(retries - 1);
                } else {
                    return None;
                }
            }

            let len = sys::mbgl_image_data_len(buffer);
            let data = sys::mbgl_image_data(buffer);
            let mut w = 0;
            let mut h = 0;
            sys::mbgl_image_size(buffer, &mut w, &mut h);

            let array: &[u8] = std::slice::from_raw_parts(data as *mut u8, len as usize);

            let img = image::RgbaImage::from_raw(w as u32, h as u32, array.to_vec());

            sys::mbgl_image_free(buffer);

            img
        }
    }

    pub fn render(&self) -> Option<image::RgbaImage> {
        self.try_render(self.render_retries)
    }

    pub fn load_style(&self, style: &str) -> &Self {
        unsafe {
            let c = std::ffi::CString::new(style).unwrap();
            sys::mbgl_map_load_style_url(self.raw, c.as_ptr());
        }
        self
    }

    pub fn set_size(&self, size: Size) -> &Self {
        unsafe {
            sys::mbgl_map_set_size(self.raw, size.0 as i32, size.1 as i32);
        }
        self
    }

    pub fn get_size(&self) -> Size {
        unsafe {
            let mut w = 0;
            let mut h = 0;
            sys::mbgl_map_get_size(self.raw, &mut w, &mut h);
            Size(w as u32, h as u32)
        }
    }

    pub fn jump_to(&self, options: &JumpToOptions) -> &Self {
        unsafe {
            let latlng = sys::mbgl_latlng_t {
                lat: options.center.0,
                lng: options.center.1,
            };
            sys::mbgl_map_jump_to(self.raw, latlng, options.zoom.unwrap_or(5.));
        };
        self
    }

    pub fn new<'a>(ty: RunLoopType, options: Option<MapOptions<'a>>) -> Option<Map> {
        let opts = options.unwrap_or_default();
        let (lraw, raw) = unsafe {
            let rloop = sys::mbgl_run_loop_create(ty as u32);

            let token = match opts.access_token {
                Some(s) => std::ffi::CString::new(s).unwrap(),
                None => std::ffi::CString::new("").unwrap(),
            };

            let map = sys::mbgl_map_create(
                opts.size.0 as i32,
                opts.size.1 as i32,
                opts.pixel_ratio.as_int(),
                token.as_ptr(),
                opts.cache_path
                    .map(|s| std::ffi::CString::new(s).unwrap().as_ptr())
                    .unwrap_or_else(|| std::ptr::null()),
                opts.assets_path
                    .map(|s| std::ffi::CString::new(s).unwrap().as_ptr())
                    .unwrap_or_else(|| std::ptr::null()),
            );

            (rloop, map)
        };

        if raw == std::ptr::null_mut() {
            return None;
        }

        Some(Map {
            lraw,
            raw,
            render_retries: opts.render_retries,
        })
    }
}
