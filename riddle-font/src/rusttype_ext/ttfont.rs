pub trait RustTypeTTFontExt {
    fn rustype_font(&self) -> &rusttype::Font<'static>;
}
