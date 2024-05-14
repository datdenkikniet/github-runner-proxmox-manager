use std::io::Write;
use std::path::{Path, PathBuf};

use proc_macro2::Literal;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::Ident;

use crate::generator::ClientModDef;

use quote::quote;

impl super::Generator {
    fn open_rw_truncate<P>(path: P) -> std::io::Result<std::fs::File>
    where
        P: AsRef<Path>,
    {
        std::fs::File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .append(false)
            .open(&path)
    }

    pub fn generate_single_file<P>(&self, output: P) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        let mut file = Self::open_rw_truncate(&output)?;
        write!(file, "{}", self.to_token_stream())
    }

    pub fn generate_file_tree<P>(self, output: P) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        let mut file = Self::open_rw_truncate(&output)?;

        let child_names = self.iter().map(|c| Ident::new(&c.name, quote!().span()));

        let child_name_literal = self.iter().map(|c| Literal::string(&c.name));

        let child_mods = quote! {
            #(
                #[cfg(feature = #child_name_literal)]
                pub mod #child_names;
            )*
        };
        write!(file, "{child_mods}")?;

        let mut buf = output.as_ref().to_path_buf();
        buf.set_extension("");

        for def in self.into_iter() {
            Self::generate_tree_impl(&buf, def)?;
        }

        Ok(())
    }

    fn generate_tree_impl(path: &PathBuf, def: ClientModDef) -> std::io::Result<()> {
        std::fs::create_dir_all(path)?;

        let my_dir = path.join(def.name);

        let my_file = my_dir.with_extension("rs");
        println!("Writing new file {:?}", my_file);
        let mut my_file = Self::open_rw_truncate(my_file)?;

        let child_names = def
            .children
            .iter()
            .map(|c| Ident::new(&c.name, quote!().span()));

        let child_mods = quote! {#(pub mod #child_names;)* };
        let body = &def.client_tokens;

        write!(my_file, "{child_mods}{body}")?;

        if !def.children.is_empty() {
            std::fs::create_dir_all(&my_dir)?;
            for def in def.children {
                Self::generate_tree_impl(&my_dir, def)?;
            }
        }

        Ok(())
    }
}
