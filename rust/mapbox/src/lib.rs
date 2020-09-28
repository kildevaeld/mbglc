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
    pub width: u32,
    pub height: u32,
    pub pixel_ratio: i32,
    pub access_token: Option<&'a str>,
    pub cache_path: Option<&'a str>,
    pub assets_path: Option<&'a str>,
}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LatLng(pub f64, pub f64);

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Size(pub u32, pub u32);

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, PartialEq)]
pub struct JumpToOptions {
    pub center: LatLng,
    pub zoom: Option<f64>,
}

impl Map {
    fn try_render(&self, retries: i32) -> Option<Vec<u8>> {
        unsafe {
            let mut len = 0;

            let buffer = sys::mbgl_map_render(self.raw, &mut len);
            if buffer == std::ptr::null_mut() {
                if retries > 0 {
                    println!("try re-render {}", retries);
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    return self.try_render(retries - 1);
                } else {
                    return None;
                }
            }

            let array: &[u8] = std::slice::from_raw_parts(buffer as *mut u8, len as usize);

            sys::free_buffer(buffer);

            Some(array.to_vec())
        }
    }

    pub fn render(&self) -> Option<image::DynamicImage> {
        let result = match self.try_render(10) {
            Some(s) => s,
            None => return None,
        };

        match image::load_from_memory_with_format(&result, image::ImageFormat::Png) {
            Ok(s) => Some(s),
            Err(_) => None,
        }
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
        let opts = options.unwrap_or_else(|| MapOptions {
            width: 1280,
            height: 1024,
            pixel_ratio: 1,
            access_token: None,
            cache_path: None,
            assets_path: None,
        });

        let (lraw, raw) = unsafe {
            let rloop = sys::mbgl_run_loop_create(ty as u32);

            let token = match opts.access_token {
                Some(s) => std::ffi::CString::new(s).unwrap(),
                None => std::ffi::CString::new("").unwrap(),
            };

            let map = sys::mbgl_map_create(
                opts.width as i32,
                opts.height as i32,
                opts.pixel_ratio,
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

        Some(Map { lraw, raw })
    }
}
