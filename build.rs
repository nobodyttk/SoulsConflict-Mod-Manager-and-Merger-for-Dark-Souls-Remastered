fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("img/icon.ico");
        res.set_language(0x0409);
        res.set("FileDescription", "SoulsConflict - Dark Souls Remastered Mod Manager & Merger");
        res.set("ProductName", "SoulsConflict");
        res.set("OriginalFilename", "SoulsConflict.exe");
        res.set("InternalName", "SoulsConflict");
        res.set("LegalCopyright", "Copyright (C) 2026 nobodyttk");
        res.set("CompanyName", "nobodyttk");
        res.set("FileVersion", "2.1.2.0");
        res.set("ProductVersion", "2.1.2.0");
        res.set_manifest_file("app.manifest");
        res.compile().unwrap();
    }
}
