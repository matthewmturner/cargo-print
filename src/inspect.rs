use syn::{File, ImplItemFn, Item, ItemFn, ItemImpl, ItemStruct};

pub struct Inspector {
    path: String,
    file: File,
}

impl Inspector {
    pub fn new(file: File, path: String) -> Self {
        Self { file, path }
    }

    pub fn inspect_file(&self) {
        for item in &self.file.items {
            match item {
                Item::Mod(module) => {
                    if let Some((_, items)) = &module.content {
                        self.inspect_items(items);
                    }
                }
                Item::Struct(s) => {
                    self.print_struct(&s);
                }
                Item::Impl(i) => {
                    self.print_impl(i);
                }
                Item::Fn(f) => self.print_fn(&f),
                _ => {}
            }
        }
    }

    pub fn inspect_items(&self, items: &[Item]) {
        for item in items {
            match item {
                Item::Mod(module) => {
                    if let Some((_, items)) = &module.content {
                        self.inspect_items(items);
                    }
                }
                Item::Fn(f) => self.print_fn(f),
                Item::Struct(s) => {
                    self.print_struct(s);
                }
                Item::Impl(i) => {
                    self.print_impl(i);
                }
                _ => {}
            }
        }
    }

    pub fn print_fn(&self, s: &ItemFn) {
        println!("Fn: {}", s.sig.ident);
    }

    pub fn print_impl_fn(&self, s: &ImplItemFn) {
        println!("  ImplFn: {}", s.sig.ident);
    }

    fn print_struct(&self, s: &ItemStruct) {
        println!("{}\tStruct: {}", self.path, s.ident);
        for field in &s.fields {
            if let Some(ident) = &field.ident {
                println!("  Field: {}", ident);
            }
        }
    }

    fn print_impl(&self, s: &ItemImpl) {
        for i in &s.items {
            match i {
                syn::ImplItem::Fn(f) => self.print_impl_fn(f),
                _ => {}
            }
        }
    }
}
